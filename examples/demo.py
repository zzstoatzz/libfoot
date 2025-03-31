from libfoot import analyze_package, get_pypi_metadata


def test_package_analysis(package_name: str, version: str | None = None):
    result = analyze_package(package_name, version)
    print(f"\nAnalyzing '{package_name}':")
    print(f"Total size: {result['total_size']:,} bytes")
    print(f"File count: {result['file_count']}")
    print("\nFile types:")
    for ext, count in result["file_types"].items():
        print(f"  {ext}: {count}")
    print("\nLargest files:")
    for file in result["largest_files"]:
        print(f"  {file['path']}: {file['size']:,} bytes")


def test_metadata(package_name: str, version: str | None = None):
    metadata = get_pypi_metadata(package_name, version)
    print(f"\nMetadata for '{package_name}':")
    print(f"Name: {metadata['name']}")
    print(f"Version: {metadata['version']}")
    print(f"Summary: {metadata['summary']}")
    print(f"Requires Python: {metadata['requires_python']}")
    print(
        f"Package size: {metadata['package_size']:,} bytes"
        if metadata["package_size"]
        else "Package size: unknown"
    )


if __name__ == "__main__":
    import sys

    package_name = sys.argv[1] if len(sys.argv) > 1 else "requests"
    version = sys.argv[2] if len(sys.argv) > 2 else None
    test_package_analysis(package_name, version)
    test_metadata(package_name, version)
