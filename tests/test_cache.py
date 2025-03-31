from libfoot import clear_cache, get_cache_stats, get_pypi_metadata


def test_cache_functionality():
    """Test that caching works correctly."""
    clear_cache()

    # Initial cache should be empty
    cache_size, oldest, newest = get_cache_stats()
    assert cache_size == 0
    assert oldest is None
    assert newest is None

    # First call should populate the cache
    get_pypi_metadata("markupsafe", "2.1.0")

    # Cache should now have an entry
    cache_size, oldest, newest = get_cache_stats()
    assert cache_size == 1
    assert oldest is not None
    assert newest is not None

    # Another different package should increase cache size
    get_pypi_metadata("requests", "2.31.0")
    cache_size, _, _ = get_cache_stats()
    assert cache_size == 2

    # Using the same parameters should use cache (no size increase)
    get_pypi_metadata("requests", "2.31.0")
    cache_size, _, _ = get_cache_stats()
    assert cache_size == 2

    # Clear cache should empty it
    clear_cache()
    cache_size, oldest, newest = get_cache_stats()
    assert cache_size == 0
    assert oldest is None
    assert newest is None
