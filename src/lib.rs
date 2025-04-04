mod analyzer;
mod cache;
mod package;

use analyzer::{fetch_pypi_metadata, PackageAnalyzer};
use cache::{clear_metadata_cache, get_cache_info};
use package::{PackageFootprint, PyPIMetadata};
use pyo3::prelude::*;
use reqwest::blocking::Client;

// Define user agent string for API requests
const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

// Create a new Client with the appropriate user agent
pub fn create_client() -> Client {
    Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .expect("Failed to build reqwest client")
}

#[pyfunction]
#[pyo3(signature = (package_name, version = None))]
fn analyze_package(package_name: &str, version: Option<String>) -> PyResult<PackageFootprint> {
    let mut analyzer = PackageAnalyzer::new();
    analyzer.analyze_package(package_name, version)
}

#[pyfunction]
#[pyo3(signature = (package_name, version = None))]
fn get_pypi_metadata(package_name: &str, version: Option<String>) -> PyResult<PyPIMetadata> {
    let client = create_client();
    fetch_pypi_metadata(&client, package_name, version)
}

/// Clears the PyPI metadata cache
#[pyfunction]
fn clear_cache() -> PyResult<()> {
    clear_metadata_cache()
}

/// Returns information about the PyPI metadata cache
///
/// Returns a tuple containing:
/// - Number of entries in cache
/// - Age of oldest entry in seconds
/// - Age of newest entry in seconds
#[pyfunction]
fn get_cache_stats() -> PyResult<(usize, Option<u64>, Option<u64>)> {
    get_cache_info()
}

#[pymodule]
#[pyo3(name = "_libfoot")]
fn libfoot(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze_package, m)?)?;
    m.add_function(wrap_pyfunction!(get_pypi_metadata, m)?)?;
    m.add_function(wrap_pyfunction!(clear_cache, m)?)?;
    m.add_function(wrap_pyfunction!(get_cache_stats, m)?)?;
    Ok(())
}
