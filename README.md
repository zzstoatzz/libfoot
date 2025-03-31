# libfoot

A lightweight Rust-powered Python library for analyzing Python package footprints.

## Features

- Fast and efficient package analysis using Rust
- Analyze package size, file types, and largest files
- Fetch PyPI metadata for packages
- Built-in caching for better performance
- Optional rich text display for beautiful output

## Installation

```bash
pip install libfoot
```

With rich display support:

```bash
pip install libfoot[display]
```

## Usage

### Basic Usage

```python
from libfoot import analyze_package, get_pypi_metadata

# Analyze a package
result = analyze_package("requests", "2.31.0")
print(f"Total size: {result['total_size']} bytes")
print(f"File count: {result['file_count']}")

# Get package metadata
metadata = get_pypi_metadata("requests", "2.31.0")
print(f"Summary: {metadata['summary']}")
```

### Rich Display (Optional)

For beautiful terminal output, install with the `display` extra and use the display functions:

```python
from libfoot import analyze_package, get_pypi_metadata, display_analysis, display_metadata

# Analyze and display with rich formatting
result = analyze_package("requests", "2.31.0")
display_analysis(result)

# Get and display metadata
metadata = get_pypi_metadata("requests", "2.31.0")
display_metadata(metadata)
```

### Cache Management

libfoot caches PyPI metadata for better performance:

```python
from libfoot import clear_cache, get_cache_stats

# Get cache statistics
stats = get_cache_stats()
print(f"Cache size: {stats.size}")
print(f"Oldest entry: {stats.oldest_entry_age} seconds")
print(f"Newest entry: {stats.newest_entry_age} seconds")

# Clear the cache
clear_cache()
```

## Configuration

libfoot can be configured using environment variables:

- `LIBFOOT_MAX_FILES`: Maximum number of largest files to track (default: 10)
- `LIBFOOT_CACHE_DURATION`: Cache duration in seconds (default: 3600)

Example:

```python
import os
os.environ["LIBFOOT_MAX_FILES"] = "20"  # Track 20 largest files
os.environ["LIBFOOT_CACHE_DURATION"] = "7200"  # Cache for 2 hours
```

## Development

To build from source:

```bash
git clone https://github.com/zzstoatzz/libfoot
cd libfoot
pip install -e ".[dev]"
```

Run tests:

```bash
pytest
```

## License

[MIT License](LICENSE)
