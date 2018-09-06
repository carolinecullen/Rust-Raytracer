#!/bin/bash
set -euo pipefail

# Repository top-level directory.
topdir=$(dirname "$0")
cd "$topdir"

if type rustfmt > /dev/null; then
    if ! "$topdir/format-all.sh" --check ; then
        echo "Formatting diffs detected! Run \"cargo fmt --all\" to correct."
        exit 1
    fi
else
    echo "rustfmt not available; formatting not checked!"
    echo
    echo "If you are using rustup, rustfmt can be installed via"
    echo "\"rustup component add --toolchain=stable rustfmt-preview\", or see"
    echo "https://github.com/rust-lang-nursery/rustfmt for more information."
fi

# Make sure the code builds in debug mode.
banner "Rust debug build"
cargo build

# Run the tests. We run these in debug mode so that assertions are enabled.
banner "Rust unit tests"
cargo test --all

banner "OK"
