import pytest
from libfoot import analyze_package, get_pypi_metadata


def test_get_metadata_success():
    """Test successful metadata retrieval for a known package."""
    metadata = get_pypi_metadata("requests")
    assert metadata["name"] == "requests"
    assert "version" in metadata
    assert "summary" in metadata
    assert "requires_dist" in metadata


def test_get_metadata_with_version():
    """Test metadata retrieval with specific version."""
    metadata = get_pypi_metadata("requests", "2.31.0")
    assert metadata["name"] == "requests"
    assert metadata["version"] == "2.31.0"


def test_get_metadata_nonexistent_package():
    """Test behavior with nonexistent package."""
    with pytest.raises(Exception):
        get_pypi_metadata("this-package-definitely-doesnt-exist-12345")


def test_analyze_package_success():
    """Test successful package analysis."""
    result = analyze_package("markupsafe", "2.1.0")  # Small, stable package
    assert result["package"]["name"] == "MarkupSafe"
    assert result["package"]["version"] == "2.1.0"
    assert result["total_size"] > 0
    assert result["file_count"] > 0
    assert len(result["file_types"]) > 0
    assert len(result["largest_files"]) > 0


def test_largest_files_sort_order():
    """Test that largest files are properly sorted."""
    result = analyze_package("requests", "2.31.0")
    files = result["largest_files"]

    # Check descending order by size
    for i in range(1, len(files)):
        assert files[i - 1]["size"] >= files[i]["size"]

    # Check maximum 10 largest files
    assert len(files) <= 10


def test_file_types_detection():
    """Test that file types are correctly detected."""
    result = analyze_package("markupsafe", "2.1.0")
    # Check that .py files are detected
    assert "py" in result["file_types"]
