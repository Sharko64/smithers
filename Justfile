set shell := ["bash", "-cu"]

run ARG:
    uv run smithers {{ARG}}

test:
    echo "Test"