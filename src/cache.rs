use crate::analyzer::PyPIResponse;
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;
use std::time::{Duration, Instant};

type CacheKey = (String, Option<String>);

#[derive(Clone)]
struct CacheEntry<T> {
    data: T,
    inserted_at: Instant,
}

type MetadataCache = RwLock<HashMap<CacheKey, CacheEntry<PyPIResponse>>>;

// Create the global static cache instance
static PYPI_METADATA_CACHE: Lazy<MetadataCache> = Lazy::new(|| RwLock::new(HashMap::new()));

fn get_cache_duration() -> Duration {
    let duration = env::var("LIBFOOT_CACHE_DURATION")
        .ok()
        .and_then(|val| val.parse::<u64>().ok())
        .unwrap_or(3600);
    Duration::from_secs(duration)
}

/// Fetches PyPI metadata with caching.
///
/// Returns cached data if available and not expired,
/// otherwise fetches fresh data from PyPI and updates the cache.
///
/// The cache expires after 1 hour to ensure reasonably fresh data.
pub fn fetch_pypi_metadata_cached(
    package_name: &str,
    version: Option<&str>,
    fetch_fn: impl FnOnce() -> PyResult<PyPIResponse>,
) -> PyResult<PyPIResponse> {
    let key = (package_name.to_string(), version.map(|s| s.to_string()));

    // Try reading from cache
    {
        let cache = PYPI_METADATA_CACHE.read().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Cache read lock poisoned")
        })?;

        if let Some(entry) = cache.get(&key) {
            if entry.inserted_at.elapsed() < get_cache_duration() {
                // Cache hit and not expired
                return Ok(entry.data.clone());
            }
            // Entry exists but expired, proceed to fetch
        }
    } // Read lock released here

    // Cache miss or expired, fetch from network
    let fetched_data = fetch_fn()?;

    // Write to cache
    {
        let mut cache = PYPI_METADATA_CACHE.write().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Cache write lock poisoned")
        })?;

        let entry = CacheEntry {
            data: fetched_data.clone(),
            inserted_at: Instant::now(),
        };

        cache.insert(key, entry);
    } // Write lock released here

    Ok(fetched_data)
}

/// Clears the PyPI metadata cache.
///
/// This can be useful for testing or forcing fresh data retrieval.
pub fn clear_metadata_cache() -> PyResult<()> {
    let mut cache = PYPI_METADATA_CACHE.write().map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Cache write lock poisoned")
    })?;

    cache.clear();
    Ok(())
}

/// Returns information about the cache.
///
/// Returns a tuple containing:
/// - Number of entries in cache
/// - Age of oldest entry in seconds
/// - Age of newest entry in seconds
pub fn get_cache_info() -> PyResult<(usize, Option<u64>, Option<u64>)> {
    let cache = PYPI_METADATA_CACHE.read().map_err(|_| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Cache read lock poisoned")
    })?;

    let cache_size = cache.len();

    let mut oldest_age = None;
    let mut newest_age = None;

    if !cache.is_empty() {
        for entry in cache.values() {
            let age = entry.inserted_at.elapsed().as_secs();

            if let Some(oldest) = oldest_age {
                if age > oldest {
                    oldest_age = Some(age);
                }
            } else {
                oldest_age = Some(age);
            }

            if let Some(newest) = newest_age {
                if age < newest {
                    newest_age = Some(age);
                }
            } else {
                newest_age = Some(age);
            }
        }
    }

    Ok((cache_size, oldest_age, newest_age))
}
