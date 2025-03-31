build:
    uvx maturin develop --uv

demo package="requests" version="2.31.0":
    uv run examples/demo.py {{package}} {{version}}

test: build
    uv run --frozen pytest -xvs tests

# Run pyright on all files
typecheck:
    uv run pyright