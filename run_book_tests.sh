#!/bin/sh
set -e

cargo build

test -d book_tests || {
    echo 'Cloning submodules.'
    git submodule update --init --recursive
}

book_tests/test_compiler target/debug/driver "$@"
