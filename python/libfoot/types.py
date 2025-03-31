from __future__ import annotations

from typing import TypedDict


class FileInfoDict(TypedDict):
    """Information about a file in the package."""

    path: str
    size: int
    file_type: str


class PackageDict(TypedDict):
    """Information about a Python package."""

    name: str
    version: str
    dependencies: list[str]


class PackageFootprintDict(TypedDict):
    """Footprint of a Python package."""

    package: PackageDict
    total_size: int
    file_count: int
    file_types: dict[str, int]
    largest_files: list[FileInfoDict]


class PyPIMetadataDict(TypedDict):
    """Metadata about a PyPI package."""

    name: str
    version: str
    summary: str
    release_url: str
    requires_python: str | None
    requires_dist: list[str]
    package_size: int | None
