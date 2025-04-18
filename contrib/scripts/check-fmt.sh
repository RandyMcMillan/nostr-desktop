#!/bin/bash

set -euo pipefail

flags=""

# Check if "check" is passed as an argument
if [[ "$#" -gt 0 && "$1" == "check" ]]; then
    flags="--check"
fi

# Install toolchain
rustup install nightly
rustup component add rustfmt --toolchain nightly

# Check workspace crates
cargo +nightly fmt --all -- --config format_code_in_doc_comments=true $flags
