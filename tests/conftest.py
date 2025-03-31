import pytest
from libfoot import analyze_package, get_pypi_metadata


@pytest.fixture(scope="session")
def markupsafe_metadata():
    """Fixture for MarkupSafe metadata - used across multiple tests."""
    return get_pypi_metadata("markupsafe", "2.1.0")


@pytest.fixture(scope="session")
def requests_metadata():
    """Fixture for Requests metadata - used across multiple tests."""
    return get_pypi_metadata("requests", "2.31.0")


@pytest.fixture(scope="session")
def markupsafe_analysis():
    """Fixture for MarkupSafe analysis - used across multiple tests."""
    return analyze_package("markupsafe", "2.1.0")


@pytest.fixture(scope="session")
def requests_analysis():
    """Fixture for Requests analysis - used across multiple tests."""
    return analyze_package("requests", "2.31.0")
