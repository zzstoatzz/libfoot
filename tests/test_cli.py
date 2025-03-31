import json
from typing import Generator
from unittest.mock import MagicMock, patch

import pytest
from libfoot.cli import main


@pytest.fixture
def mock_analyze_package():
    with patch("libfoot.cli.analyze_package") as mock:
        mock.return_value = {
            "package": {"name": "testpkg", "version": "1.0.0", "dependencies": []},
            "total_size": 1000,
            "file_count": 10,
            "file_types": {"py": 8, "txt": 2},
            "largest_files": [
                {"path": "testpkg/main.py", "size": 500, "file_type": "py"}
            ],
        }
        yield mock


@pytest.fixture
def mock_get_pypi_metadata() -> Generator[MagicMock, None, None]:
    with patch("libfoot.cli.get_pypi_metadata") as mock:
        mock.return_value = {
            "name": "testpkg",
            "version": "1.0.0",
            "summary": "Test package",
            "release_url": "https://example.com/testpkg",
            "requires_python": ">=3.8",
            "requires_dist": [],
            "package_size": 1000,
        }
        yield mock


@pytest.fixture
def mock_display_functions() -> Generator[dict[str, MagicMock], None, None]:
    with patch("libfoot.cli.display_analysis") as analyze_mock, patch(
        "libfoot.cli.display_metadata"
    ) as metadata_mock:
        yield {"analyze": analyze_mock, "metadata": metadata_mock}


def test_analyze_command(
    mock_analyze_package: MagicMock, mock_display_functions: dict[str, MagicMock]
):
    # Test analyze command with display
    exit_code = main(["analyze", "testpkg"])

    # Check return code and function calls
    assert exit_code == 0
    mock_analyze_package.assert_called_once_with("testpkg", None)
    mock_display_functions["analyze"].assert_called_once_with(
        mock_analyze_package.return_value
    )


def test_analyze_command_with_json(
    mock_analyze_package: MagicMock,
    mock_display_functions: dict[str, MagicMock],
    capsys: pytest.CaptureFixture[str],
):
    exit_code = main(["analyze", "testpkg", "--json"])

    # Check return code and function calls
    assert exit_code == 0
    mock_analyze_package.assert_called_once_with("testpkg", None)
    mock_display_functions["analyze"].assert_not_called()

    # Check JSON output
    captured = capsys.readouterr()
    output = json.loads(captured.out)
    assert output["package"]["name"] == "testpkg"
    assert output["total_size"] == 1000


def test_metadata_command(
    mock_get_pypi_metadata: MagicMock, mock_display_functions: dict[str, MagicMock]
):
    # Test metadata command with version
    exit_code = main(["metadata", "testpkg", "-v", "1.0.0"])

    # Check return code and function calls
    assert exit_code == 0
    mock_get_pypi_metadata.assert_called_once_with("testpkg", "1.0.0")
    mock_display_functions["metadata"].assert_called_once_with(
        mock_get_pypi_metadata.return_value
    )


def test_error_handling():
    # Test with an invalid command that will raise an exception
    with patch("libfoot.cli.analyze_package", side_effect=ValueError("Test error")):
        exit_code = main(["analyze", "badpkg"])
        assert exit_code == 1  # Should return error code
