#!/bin/sh

exec cargo run \
    --quiet \
    --release \
    --target-dir=/tmp/bittorrent \
    --manifest-path $(dirname "$0")/Cargo.toml -- "$@"
