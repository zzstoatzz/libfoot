# Libfoot Tests

This directory contains the test suite for libfoot. Tests are organized by functionality to make them easier to maintain.

## Test Organization

- `test_metadata.py`: Tests for package metadata retrieval
- `test_analyzer.py`: Tests for package analysis functionality
- `test_cache.py`: Tests for caching behavior
- `test_env_vars.py`: Tests for environment variable configurations
- `conftest.py`: Shared test fixtures

## Running Tests

To run the full test suite:

```bash
pytest
```

For running specific test files:

```bash
pytest tests/test_metadata.py
```

## Performance Optimization

The test suite uses fixtures with session scope to minimize redundant API calls and package downloads, making tests run faster.

To run tests in parallel for even faster execution:

```bash
pip install pytest-xdist
pytest -n auto  # Uses available CPU cores 