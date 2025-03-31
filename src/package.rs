use pyo3::prelude::IntoPyObject;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, IntoPyObject)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
}

impl Package {
    pub fn new(name: String, version: String, dependencies: Vec<String>) -> Self {
        Self {
            name,
            version,
            dependencies,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, IntoPyObject)]
pub struct PackageFootprint {
    pub package: Package,
    pub total_size: u64,
    pub file_count: usize,
    pub file_types: HashMap<String, usize>,
    pub largest_files: Vec<FileInfo>,
}

impl PackageFootprint {
    pub fn new(package: Package) -> Self {
        Self {
            package,
            total_size: 0,
            file_count: 0,
            file_types: HashMap::new(),
            largest_files: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, IntoPyObject)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub file_type: String,
}

impl FileInfo {
    pub fn new(path: String, size: u64, file_type: String) -> Self {
        Self {
            path,
            size,
            file_type,
        }
    }
}

// Implement comparison traits for FileInfo
impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.path == other.path
    }
}

impl Eq for FileInfo {}

impl PartialOrd for FileInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by size
        match self.size.cmp(&other.size) {
            Ordering::Equal => self.path.cmp(&other.path), // If same size, compare by path
            other_ordering => other_ordering,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, IntoPyObject)]
pub struct PyPIMetadata {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub release_url: String,
    pub requires_python: Option<String>,
    pub requires_dist: Vec<String>,
    pub package_size: Option<u64>,
}
