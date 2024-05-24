#!/bin/bash

if [[ "${UTILITY_RELEASE}" == "true" ]]; then
    echo "Test with release version of borsh and unc-vm-logic"
    sed -n '/^borsh/p' unc-sdk/Cargo.toml 
    sed -n '/^unc-vm-logic/p' unc-sdk/Cargo.toml
    cargo test --all
else
    echo "Test with git version of borsh and unc-vm-logic"

    cargo generate-lockfile

    cp Cargo.toml{,.bak}
    cp Cargo.lock{,.bak}

    sed -i "" "s|###||g" Cargo.toml
    
    set +e
    cargo test --all
    status=$?
    set -e

    mv Cargo.toml{.bak,}
    mv Cargo.lock{.bak,}
    if [ $status -ne 0 ]; then
      exit $status
    fi

    # Only testing it for one configuration to avoid running the same tests twice
    echo "Build wasm32 for all examples"

    ./examples/build_all_docker.sh --check
    echo "Testing all examples"
    ./examples/test_all.sh
    ./examples/size_all.sh
fi
