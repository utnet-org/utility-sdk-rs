#!/usr/bin/env bash
set -ex
for p in unc-sys unc-sdk-macros unc-sdk unc-contract-standards
do
pushd ./${p}
cargo publish
popd
# Sleep a bit to let the previous package upload to crates.io. Otherwise we fail publishing checks.
sleep 30
done
