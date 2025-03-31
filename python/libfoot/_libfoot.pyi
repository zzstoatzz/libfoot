from __future__ import annotations

from libfoot.types import PackageFootprintDict, PyPIMetadataDict

def analyze_package(
    package_name: str, version: str | None = None
) -> PackageFootprintDict:
    """
    Analyze a Python package by downloading and examining its wheel file.

    Args:
        package_name: Name of the package to analyze
        version: Optional specific version to analyze

    Returns:
        A dictionary containing package analysis results
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
        A dictionary containing package metadata
    """
    ...

def clear_cache() -> None:
    """
    Clear the PyPI metadata cache.
    """
    ...

def get_cache_stats() -> tuple[int, int | None, int | None]:
    """
    Get statistics about the PyPI metadata cache.
    """
    ...
