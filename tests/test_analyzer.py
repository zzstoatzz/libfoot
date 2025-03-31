from libfoot.types import PackageFootprintDict


def test_analyze_package_success(markupsafe_analysis: PackageFootprintDict):
    """Test successful package analysis."""
    result = markupsafe_analysis
    assert result["package"]["name"] == "MarkupSafe"
    assert result["package"]["version"] == "2.1.0"
    assert result["total_size"] > 0
    assert result["file_count"] > 0
    assert len(result["file_types"]) > 0
    assert len(result["largest_files"]) > 0


def test_largest_files_sort_order(requests_analysis: PackageFootprintDict):
    """Test that largest files are properly sorted."""
    files = requests_analysis["largest_files"]

    # Check files are sorted by size in descending order
    for i in range(1, len(files)):
        assert files[i - 1]["size"] >= files[i]["size"], (
            f"Files not in descending size order: {files[i - 1]['size']} before {files[i]['size']}"
        )

    # Check maximum 10 largest files by default
    assert len(files) <= 10


def test_file_types_detection(markupsafe_analysis: PackageFootprintDict):
    """Test that file types are correctly detected."""
    result = markupsafe_analysis
    # Check that .py files are detected
    assert "py" in result["file_types"]
