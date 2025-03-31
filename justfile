build:
    uvx maturin develop --uv

demo package="requests" version="2.31.0":
    uv run --extra display examples/rich_demo.py {{package}} {{version}}

test: build
    uv run --frozen pytest -xvs tests
