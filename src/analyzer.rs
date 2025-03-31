use crate::cache::fetch_pypi_metadata_cached;
use crate::package::{FileInfo, Package, PackageFootprint, PyPIMetadata};
use pyo3::prelude::*;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::io::Read;
use std::path::Path;
use tempfile::NamedTempFile;
use zip::ZipArchive;

// Default value for maximum number of largest files to track
const DEFAULT_MAX_FILES: usize = 10;

// Define user agent string for API requests
const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

// Read MAX_LARGEST_FILES from environment variable or use default
/// Returns the maximum number of largest files to track.
///
/// Uses the LIBFOOT_MAX_FILES environment variable if set,
/// otherwise falls back to DEFAULT_MAX_FILES (10).
fn get_max_files() -> usize {
    env::var("LIBFOOT_MAX_FILES")
        .ok()
        .and_then(|val| val.parse::<usize>().ok())
        .unwrap_or(DEFAULT_MAX_FILES)
}

#[derive(Default)]
pub struct PackageAnalyzer {
    client: Option<Client>,
}

#[derive(Deserialize, Clone)]
pub struct PyPIResponse {
    pub info: PyPIInfo,
    pub urls: Vec<PyPIFileInfo>,
}

#[derive(Deserialize, Clone)]
pub struct PyPIInfo {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub requires_python: Option<String>,
    pub requires_dist: Option<Vec<String>>,
    pub project_urls: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Clone)]
pub struct PyPIFileInfo {
    pub url: String,
    pub packagetype: String,
    pub size: u64,
}

impl PackageAnalyzer {
    pub fn new() -> Self {
        Self {
            client: Some(
                Client::builder()
                    .user_agent(APP_USER_AGENT)
                    .build()
                    .expect("Failed to build reqwest client"),
            ),
        }
    }

    fn get_client(&mut self) -> &Client {
        if self.client.is_none() {
            self.client = Some(
                Client::builder()
                    .user_agent(APP_USER_AGENT)
                    .build()
                    .expect("Failed to build reqwest client"),
            );
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

        // Get max files configuration once
        let max_files = get_max_files();

        // Use a min-heap to track the largest files
        let mut largest_files_heap: BinaryHeap<Reverse<FileInfo>> =
            BinaryHeap::with_capacity(max_files + 1);

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
                if largest_files_heap.len() < max_files {
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

        // Collect files from the heap, unwrapping the Reverse
        let mut largest_files: Vec<FileInfo> = largest_files_heap
            .into_iter()
            .map(|Reverse(file_info)| file_info)
            .collect();

        // Sort explicitly by size in descending order
        largest_files.sort_by(|a, b| b.size.cmp(&a.size));

        footprint.largest_files = largest_files;

        Ok(footprint)
    }

    /// Analyzes a Python package from PyPI.
    ///
    /// Downloads and analyzes the wheel file for the specified package
    /// and returns a PackageFootprint with the analysis results.
    ///
    /// The number of largest files tracked can be configured using the
    /// LIBFOOT_MAX_FILES environment variable.
    pub fn analyze_package(
        &mut self,
        package_name: &str,
        version: Option<String>,
    ) -> PyResult<PackageFootprint> {
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

        let wheel_url = wheel_info.url.clone();

        // 3. Download wheel
        let temp_file = self.download_wheel(&wheel_url)?;

        // 4. Create Package instance with dependencies
        let package = Package::new(
            pypi_data.info.name.clone(),
            pypi_data.info.version.clone(),
            pypi_data.info.requires_dist.clone().unwrap_or_default(),
        );

        // 5. Analyze wheel contents
        let file = std::fs::File::open(temp_file.path())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        let archive = ZipArchive::new(file)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        let footprint = self.analyze_wheel_contents(package, archive)?;

        // Return the struct directly, PyO3 will handle the conversion
        Ok(footprint)
    }
}

pub fn fetch_pypi_metadata_raw(
    client: &Client,
    package_name: &str,
    version: Option<&str>,
) -> PyResult<PyPIResponse> {
    // Use the caching function
    fetch_pypi_metadata_cached(package_name, version, || {
        // This closure is the actual fetch function
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
    })
}

pub fn fetch_pypi_metadata(package_name: &str, version: Option<String>) -> PyResult<PyPIMetadata> {
    let client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("Failed to build reqwest client");
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
        package_size: pypi_data
            .urls
            .iter()
            .find(|f| f.packagetype == "bdist_wheel")
            .map(|f| f.size),
    })
}
