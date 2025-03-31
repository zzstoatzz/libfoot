from __future__ import annotations

from typing import NamedTuple

from ._libfoot import (
    analyze_package,
    clear_cache,
    get_pypi_metadata,
)
from ._libfoot import get_cache_stats as _get_cache_stats
from .display import display_analysis, display_metadata


class CacheStats(NamedTuple):
    """Statistics about the PyPI metadata cache."""

    size: int
    oldest_entry_age: int | None
    newest_entry_age: int | None


def get_cache_stats() -> CacheStats:
    """
    Get statistics about the PyPI metadata cache.

    Returns:
        A named tuple containing:
        - size: Number of entries in cache
        - oldest_entry_age: Age of oldest entry in seconds (None if cache is empty)
        - newest_entry_age: Age of newest entry in seconds (None if cache is empty)
    """
    size, oldest, newest = _get_cache_stats()
    return CacheStats(size, oldest, newest)


__all__ = [
    "analyze_package",
    "clear_cache",
    "get_cache_stats",
    "get_pypi_metadata",
    "CacheStats",
    "display_analysis",
    "display_metadata",
]
