setup-dev:
    uv venv
    uv sync --dev
    maturin develop --uv