build:
    uvx maturin develop --uv

demo package="requests" version="2.31.0":
    uv run test_libfoot.py {{package}} {{version}}

test:
    uv run --frozen pytest -xvs tests
