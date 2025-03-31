import pytest
from libfoot import get_pypi_metadata


def test_get_metadata_success(requests_metadata):
    """Test successful metadata retrieval for a known package."""
    metadata = requests_metadata
    assert metadata["name"] == "requests"
    assert "version" in metadata
    assert "summary" in metadata
    assert "requires_dist" in metadata


def test_get_metadata_with_version(requests_metadata):
    """Test metadata retrieval with specific version."""
    metadata = requests_metadata
    assert metadata["name"] == "requests"
    assert metadata["version"] == "2.31.0"


def test_get_metadata_nonexistent_package():
    """Test behavior with nonexistent package."""
    with pytest.raises(Exception):
        get_pypi_metadata("this-package-definitely-doesnt-exist-12345")
