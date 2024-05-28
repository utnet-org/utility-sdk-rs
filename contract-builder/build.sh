#!/bin/bash
set -eox pipefail

if [ "$1" != "amd64" ] && [ "$1" != "arm64" ]; then
    echo " Please enter one of amd64 or arm64"
    exit 1
fi

branch=${GH_BRANCH//:/_}
branch=${branch//\//_}
commit=${GH_COMMIT}
if [[ ${commit} == "HEAD" ]]; then
    commit=$(git rev-parse HEAD)
fi

image_name="contract-builder"

if docker buildx ls | grep -q contract-builder; then
    docker buildx use --builder contract-builder
else
    docker buildx create --name contract-builder --use
fi

if [[ ${branch} == "main" ]]; then
    docker buildx build --platform linux/$1 -t ruster2024/"${image_name}:${branch}-${commit}-$1" -t ruster2024/${image_name}:latest-$1 --push .
else
    docker buildx build --platform linux/$1 -t ruster2024/"${image_name}:${branch}-${commit}-$1" --push .
fi
