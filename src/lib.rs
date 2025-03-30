mod analyzer;
mod package;

use analyzer::{fetch_pypi_metadata, PackageAnalyzer};
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyfunction]
#[pyo3(signature = (package_name, version = None))]
fn analyze_package(package_name: &str, version: Option<String>) -> PyResult<PyObject> {
    let mut analyzer = PackageAnalyzer::new();
    analyzer.analyze_package(package_name, version)
}

#[pyfunction]
#[pyo3(signature = (package_name, version = None))]
fn get_pypi_metadata(package_name: &str, version: Option<String>) -> PyResult<PyObject> {
    let metadata = fetch_pypi_metadata(package_name, version)?;

    // Convert to PyObject manually
    Python::with_gil(|py| {
        let dict = PyDict::new(py);
        dict.set_item("name", &metadata.name)?;
        dict.set_item("version", &metadata.version)?;
        dict.set_item("summary", &metadata.summary)?;
        dict.set_item("release_url", &metadata.release_url)?;

        if let Some(requires_python) = &metadata.requires_python {
            dict.set_item("requires_python", requires_python)?;
        } else {
            dict.set_item("requires_python", py.None())?;
        }

        dict.set_item("requires_dist", &metadata.requires_dist)?;

        if let Some(size) = metadata.package_size {
            dict.set_item("package_size", size)?;
        } else {
            dict.set_item("package_size", py.None())?;
        }

        Ok(dict.into())
    })
}

#[pymodule]
#[pyo3(name = "_libfoot")]
fn libfoot(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze_package, m)?)?;
    m.add_function(wrap_pyfunction!(get_pypi_metadata, m)?)?;
    Ok(())
}
