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
