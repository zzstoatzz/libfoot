# `libfoot`

[![PyPI](https://img.shields.io/pypi/v/libfoot)](https://pypi.org/project/libfoot/)
[![Python Versions](https://img.shields.io/pypi/pyversions/libfoot)](https://pypi.org/project/libfoot/)
[![Tests](https://github.com/zzstoatzz/libfoot/actions/workflows/test.yml/badge.svg)](https://github.com/zzstoatzz/libfoot/actions/workflows/test.yml)
[![Lint](https://github.com/zzstoatzz/libfoot/actions/workflows/lint.yml/badge.svg)](https://github.com/zzstoatzz/libfoot/actions/workflows/lint.yml)
[![License](https://img.shields.io/github/license/zzstoatzz/libfoot)](https://github.com/zzstoatzz/libfoot/blob/main/LICENSE)

a tool for analyzing the footprint of a Python package, written in Rust with python bindings via [`pyo3`](https://github.com/pyo3/pyo3)

## installation
Add `libfoot` to your project:
```bash
uv add libfoot
```

## usage

if you have `uv` installed, you don't need a static installation of `libfoot`:

```bash
uvx libfoot analyze pydantic -v 2.11.1
```

### CLI

libfoot provides a command-line interface for easy access to its functionality:

```bash
# analyze a package
libfoot analyze requests

# specify a version
libfoot analyze requests -v 2.31.0 

# get package metadata
libfoot metadata requests

# output in JSON format
libfoot analyze requests --json
```


### python interface

```python
from libfoot import analyze_package, get_pypi_metadata

# analyze a package
result = analyze_package("requests", "2.31.0")
print(f"Total size: {result['total_size']} bytes")
print(f"File count: {result['file_count']}")

# get package metadata
metadata = get_pypi_metadata("requests", "2.31.0")
print(f"Summary: {metadata['summary']}")
```

## development

To build from source:

```bash
git clone https://github.com/zzstoatzz/libfoot
cd libfoot
uv sync
```

I use [`just`](https://github.com/casey/just) as a command runner:

```bash
# build the project
just build

# run tests (builds beforehand automatically)
just test

# typecheck the project
just typecheck

# run the demo
just demo
```
## license

[MIT License](LICENSE)

## feedback

If you have any feedback, please [open an issue](https://github.com/zzstoatzz/libfoot/issues) or [create a discussion](https://github.com/zzstoatzz/libfoot/discussions).