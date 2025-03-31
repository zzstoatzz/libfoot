import os
import time
from typing import Generator

import pytest
from libfoot import analyze_package, clear_cache, get_cache_stats, get_pypi_metadata


@pytest.fixture
def original_max_files_env_var() -> Generator[None, None, None]:
    """Fixture to save and restore LIBFOOT_MAX_FILES environment variable."""
    original_value = os.environ.get("LIBFOOT_MAX_FILES")
    yield
    if original_value is None:
        os.environ.pop("LIBFOOT_MAX_FILES", None)
    else:
        os.environ["LIBFOOT_MAX_FILES"] = original_value


@pytest.fixture
def original_cache_duration_env_var() -> Generator[None, None, None]:
    """Fixture to save and restore LIBFOOT_CACHE_DURATION environment variable."""
    original_value = os.environ.get("LIBFOOT_CACHE_DURATION")
    yield
    if original_value is None:
        os.environ.pop("LIBFOOT_CACHE_DURATION", None)
    else:
        os.environ["LIBFOOT_CACHE_DURATION"] = original_value


@pytest.mark.usefixtures("original_max_files_env_var")
def test_max_files_env_var(monkeypatch: pytest.MonkeyPatch):
    """Test that LIBFOOT_MAX_FILES environment variable controls the number of files returned."""
    monkeypatch.setenv("LIBFOOT_MAX_FILES", "3")

    result = analyze_package("requests", "2.31.0")

    assert len(result["largest_files"]) == 3

    monkeypatch.setenv("LIBFOOT_MAX_FILES", "5")

    result = analyze_package("requests", "2.31.0")

    assert len(result["largest_files"]) == 5


@pytest.mark.usefixtures("original_cache_duration_env_var")
def test_cache_duration_env_var(monkeypatch: pytest.MonkeyPatch):
    """Test that LIBFOOT_CACHE_DURATION environment variable controls cache expiration time."""
    # Set a very short cache duration (2 seconds)
    monkeypatch.setenv("LIBFOOT_CACHE_DURATION", "2")

    # Clear cache to start fresh
    clear_cache()

    # First request populates the cache
    get_pypi_metadata("markupsafe", "2.1.0")

    # Verify cache has an entry
    stats = get_cache_stats()
    assert stats.size == 1

    # Wait for enough time for the cache to expire
    time.sleep(3)

    # Make the same request, which should fetch fresh data since cache expired
    get_pypi_metadata("markupsafe", "2.1.0")

    # The cache size should still be 1, but the cache entry should be newer
    new_stats = get_cache_stats()
    assert new_stats.size == 1

    # The newest entry should be newer than before
    assert new_stats.newest_entry_age is not None and new_stats.newest_entry_age < 3
