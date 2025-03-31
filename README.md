# `libfoot`

a tool for analyzing the footprint of a Python package, written in Rust with python bindings via [`pyo3`](https://github.com/pyo3/pyo3)

## Installation
Add `libfoot` to your project:
```bash
uv add libfoot
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

### Rich Display

libfoot includes rich display functionality:

```python
from libfoot import analyze_package, get_pypi_metadata
from libfoot.display import display_analysis, display_metadata

# Analyze and display with rich formatting
result = analyze_package("requests", "2.31.0")
display_analysis(result)

# Get and display metadata
metadata = get_pypi_metadata("requests", "2.31.0")
display_metadata(metadata)
```

### Command Line Interface

libfoot provides a command-line interface for easy access to its functionality:

```bash
# Analyze a package (with rich output)
libfoot analyze requests

# Specify a version
libfoot analyze requests -v 2.31.0 

# Get package metadata
libfoot metadata requests

# Output in JSON format
libfoot analyze requests --json
```

### Cache Management

libfoot caches PyPI metadata within a Python process for better performance:

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
uv sync
```

Run tests:

```bash
uv run --frozen pytest
```

The project uses GitHub Actions for automated linting and testing across multiple Python versions and platforms.

## License

[MIT License](LICENSE)
