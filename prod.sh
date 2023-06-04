#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

pushd frontend
CARGO_TARGET_DIR=../target-trunk trunk build --release --public-url /
popd

cargo run --bin backend --release -- --static-dir ./dist
