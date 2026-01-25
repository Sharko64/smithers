set shell := ["bash", "-cu"]

run ARG:
    python3 -m src.smithers {{ARG}}

test:
    echo "Test"