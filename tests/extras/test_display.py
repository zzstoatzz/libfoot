"""Test the rich display functionality of libfoot."""

import io
from contextlib import redirect_stdout

import pytest
from libfoot.display import display_analysis, display_metadata
from libfoot.types import PackageFootprintDict, PyPIMetadataDict


def test_display_module_functions():
    """Test that the display module contains the expected functions."""

    # Verify the functions exist
    assert callable(display_analysis)
    assert callable(display_metadata)


@pytest.fixture
def mock_analysis() -> PackageFootprintDict:
    return {
        "package": {"name": "test", "version": "1.0.0", "dependencies": []},
        "total_size": 1000,
        "file_count": 10,
        "file_types": {"py": 5, "md": 3, "unknown": 2},
        "largest_files": [{"path": "test.py", "size": 500, "file_type": "py"}],
    }


@pytest.fixture
def mock_metadata() -> PyPIMetadataDict:
    return {
        "name": "test",
        "version": "1.0.0",
        "summary": "Test package",
        "release_url": "https://pypi.org/project/test/1.0.0/",
        "requires_python": ">=3.7",
        "requires_dist": ["requests>=2.0.0"],
        "package_size": 1000,
    }


def test_display_analysis_function(mock_analysis: PackageFootprintDict):
    """Test the display_analysis function with captured stdout."""
    # Capture stdout to prevent output during tests
    f = io.StringIO()
    with redirect_stdout(f):
        display_analysis(mock_analysis)

    # Verify expected content in output
    output = f.getvalue()
    assert "Analysis for test v1.0.0" in output
    assert "Total Size: 1,000 bytes" in output
    assert "File Count: 10" in output
    assert "File Types" in output
    assert "Largest Files" in output


def test_display_metadata_function(mock_metadata: PyPIMetadataDict):
    """Test the display_metadata function with captured stdout."""
    # Capture stdout to prevent output during tests
    f = io.StringIO()
    with redirect_stdout(f):
        display_metadata(mock_metadata)

    # Verify expected content in output
    output = f.getvalue()
    assert "PyPI Metadata for test v1.0.0" in output
    assert "Summary: Test package" in output
    assert "Requires Python: >=3.7" in output
    assert "Package Size: 1,000 bytes" in output
    assert "Dependencies:" in output
    assert "requests>=2.0.0" in output
