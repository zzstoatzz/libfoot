from __future__ import annotations

from typing import TypedDict

class FileInfoDict(TypedDict):
    path: str
    size: int
    file_type: str

class PackageDict(TypedDict):
    name: str
    version: str
    dependencies: list[str]

class PackageFootprintDict(TypedDict):
    package: PackageDict
    total_size: int
    file_count: int
    file_types: dict[str, int]
    largest_files: list[FileInfoDict]

class PyPIMetadataDict(TypedDict):
    name: str
    version: str
    summary: str
    release_url: str
    requires_python: str | None
    requires_dist: list[str]
    package_size: int | None

def analyze_package(
    package_name: str, version: str | None = None
) -> PackageFootprintDict:
    """
    Analyze a Python package by downloading and examining its wheel file.

    Args:
        package_name: Name of the package to analyze
        version: Optional specific version to analyze

    Returns:
        A dictionary containing:
        - package: Information about the package (name, version)
        - total_size: Total size in bytes
        - file_count: Number of files in the package
        - file_types: Count of files by extension
        - largest_files: List of largest files with their sizes
    """
    ...

def get_pypi_metadata(
    package_name: str, version: str | None = None
) -> PyPIMetadataDict:
    """
    Fetch metadata for a package from PyPI.

    Args:
        package_name: Name of the package to get metadata for
        version: Optional specific version to fetch

    Returns:
        A dictionary containing package metadata:
        - name: Package name
        - version: Package version
        - summary: Short description
        - release_url: URL to the package homepage
        - requires_python: Python version requirement
        - requires_dist: List of package dependencies
        - package_size: Size of the package in bytes
    """
    ...
