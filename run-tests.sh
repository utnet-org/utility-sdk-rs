#!/bin/bash

set -e

cargo fmt --check --all
cargo clippy --tests --all-features -- -Dclippy::all
cargo test --all --features unstable,legacy

# Only testing it for one configuration to avoid running the same tests twice
echo "Build wasm32 for all examples"

#./examples/build_all_docker.sh --check
echo "Testing all examples"
./examples/test_all.sh
./examples/size_all.sh
