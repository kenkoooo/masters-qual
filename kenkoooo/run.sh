#!/bin/sh

set -eu

mkdir -p out
rm -rf out/*

cargo build --release

for input in `ls ../generator/in`;
do
    echo "Running test $input"
    cargo run --bin solver < ../generator/in/$input > out/$input
    echo "Validating $input"
    cargo run --bin validator ../generator/in/$input ./out/$input
done
