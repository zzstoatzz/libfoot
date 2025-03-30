use crate::package::{FileInfo, Package, PackageFootprint, PyPIMetadata};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::io::Read;
use std::path::Path;
use tempfile::NamedTempFile;
use zip::ZipArchive;

const MAX_LARGEST_FILES: usize = 10;

#[pyclass]
#[derive(Default)]
pub struct PackageAnalyzer {
    client: Option<Client>,
}

#[derive(Deserialize)]
pub struct PyPIResponse {
    info: PyPIInfo,
    urls: Vec<PyPIFileInfo>,
}

#[derive(Deserialize)]
pub struct PyPIInfo {
    name: String,
    version: String,
    summary: String,
    requires_python: Option<String>,
    requires_dist: Option<Vec<String>>,
    project_urls: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
pub struct PyPIFileInfo {
    url: String,
    packagetype: String,
    size: u64,
}

impl PackageAnalyzer {
    fn get_client(&mut self) -> &Client {
        if self.client.is_none() {
            self.client = Some(Client::new());
        }
        self.client.as_ref().unwrap()
    }

    fn find_wheel_url<'a>(&self, files: &'a [PyPIFileInfo]) -> Option<&'a PyPIFileInfo> {
        // Prioritize wheels (.whl files)
        files.iter().find(|f| f.packagetype == "bdist_wheel")
    }

    fn download_wheel(&mut self, url: &str) -> PyResult<NamedTempFile> {
        let client = self.get_client();

        let mut temp_file = tempfile::Builder::new()
            .suffix(".whl")
            .tempfile()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        let mut response = client
            .get(url)
            .send()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyConnectionError, _>(e.to_string()))?;

        if !response.status().is_success() {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to download wheel: {} ({})",
                url,
                response.status()
            )));
        }

        response
            .copy_to(&mut temp_file)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        Ok(temp_file)
    }

    fn analyze_wheel_contents<R: Read + std::io::Seek>(
        &self,
        package: Package,
        mut archive: ZipArchive<R>,
    ) -> PyResult<PackageFootprint> {
        let mut footprint = PackageFootprint::new(package);
        let mut file_types = HashMap::new();

        // Use a min-heap to track the largest files
        let mut largest_files_heap: BinaryHeap<Reverse<FileInfo>> =
            BinaryHeap::with_capacity(MAX_LARGEST_FILES + 1);

        for i in 0..archive.len() {
            let file = archive.by_index(i).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Zip error: {}", e))
            })?;

            if file.is_file() {
                footprint.file_count += 1;
                let size = file.size();
                footprint.total_size += size;

                let path = file.name().to_string();

                let ext = Path::new(&path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                *file_types.entry(ext.clone()).or_insert(0) += 1;

                let file_info = FileInfo::new(path, size, ext);

                // Efficiently maintain top K largest files
                if largest_files_heap.len() < MAX_LARGEST_FILES {
                    largest_files_heap.push(Reverse(file_info));
                } else if let Some(Reverse(smallest)) = largest_files_heap.peek() {
                    if file_info.size > smallest.size {
                        largest_files_heap.pop();
                        largest_files_heap.push(Reverse(file_info));
                    }
                }
            }
        }

        footprint.file_types = file_types;

        // Extract files from heap and sort
        footprint.largest_files = largest_files_heap
            .into_iter()
            .map(|Reverse(file_info)| file_info)
            .collect::<Vec<_>>();

        // Sort in descending order of size (largest first)
        footprint.largest_files.sort_by(|a, b| b.size.cmp(&a.size));

        Ok(footprint)
    }
}

#[pymethods]
impl PackageAnalyzer {
    #[new]
    pub fn new() -> Self {
        Self {
            client: Some(Client::new()),
        }
    }

    pub fn analyze_package(
        &mut self,
        package_name: &str,
        version: Option<String>,
    ) -> PyResult<PyObject> {
        // 1. Fetch PyPI metadata
        let client = self.get_client();
        let pypi_data = fetch_pypi_metadata_raw(client, package_name, version.as_deref())?;

        // 2. Find wheel URL
        let wheel_info = self.find_wheel_url(&pypi_data.urls).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "No wheel found for package: {}, version: {}",
                package_name,
                version.as_deref().unwrap_or(&pypi_data.info.version)
            ))
        })?;

        // Clone the URL to avoid reference issues
        let wheel_url = wheel_info.url.clone();

        // 3. Download wheel
        let temp_file = self.download_wheel(&wheel_url)?;

        // 4. Create Package instance
        let package = Package::new(pypi_data.info.name.clone(), pypi_data.info.version.clone());

        // 5. Analyze wheel contents
        let file = std::fs::File::open(temp_file.path())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        let archive = ZipArchive::new(file)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        let footprint = self.analyze_wheel_contents(package, archive)?;

        // Convert to PyObject manually
        Python::with_gil(|py| {
            let dict = PyDict::new(py);

            // Package info
            let pkg_dict = PyDict::new(py);
            pkg_dict.set_item("name", &footprint.package.name)?;
            pkg_dict.set_item("version", &footprint.package.version)?;
            pkg_dict.set_item("dependencies", &footprint.package.dependencies)?;
            dict.set_item("package", pkg_dict)?;

            // Stats
            dict.set_item("total_size", footprint.total_size)?;
            dict.set_item("file_count", footprint.file_count)?;

            // File types
            let file_types_dict = PyDict::new(py);
            for (ext, count) in &footprint.file_types {
                file_types_dict.set_item(ext, count)?;
            }
            dict.set_item("file_types", file_types_dict)?;

            // Largest files
            let largest_files = pyo3::types::PyList::empty(py);
            for file in &footprint.largest_files {
                let file_dict = PyDict::new(py);
                file_dict.set_item("path", &file.path)?;
                file_dict.set_item("size", file.size)?;
                file_dict.set_item("file_type", &file.file_type)?;
                largest_files.append(file_dict)?;
            }
            dict.set_item("largest_files", largest_files)?;

            Ok(dict.into())
        })
    }
}

pub fn fetch_pypi_metadata_raw(
    client: &Client,
    package_name: &str,
    version: Option<&str>,
) -> PyResult<PyPIResponse> {
    let url = match version {
        Some(ver) => format!("https://pypi.org/pypi/{}/{}/json", package_name, ver),
        None => format!("https://pypi.org/pypi/{}/json", package_name),
    };

    let response = client
        .get(&url)
        .send()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyConnectionError, _>(e.to_string()))?;

    if !response.status().is_success() {
        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to fetch PyPI metadata: {}",
            response.status()
        )));
    }

    response
        .json::<PyPIResponse>()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

pub fn fetch_pypi_metadata(package_name: &str, version: Option<String>) -> PyResult<PyPIMetadata> {
    let client = Client::new();
    let pypi_data = fetch_pypi_metadata_raw(&client, package_name, version.as_deref())?;

    let requires_dist = pypi_data.info.requires_dist.unwrap_or_default();

    Ok(PyPIMetadata {
        name: pypi_data.info.name,
        version: pypi_data.info.version,
        summary: pypi_data.info.summary,
        release_url: pypi_data
            .info
            .project_urls
            .and_then(|urls| urls.get("Homepage").cloned())
            .unwrap_or_default(),
        requires_python: pypi_data.info.requires_python,
        requires_dist,
        package_size: pypi_data.urls.first().map(|f| f.size),
        raw_data: HashMap::new(),
    })
}
