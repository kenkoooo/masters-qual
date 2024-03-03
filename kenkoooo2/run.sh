#!/bin/bash

set -euxo pipefail

if [ -d out ]; then
    echo "out directory exists"
    if [ -d out.bk ]; then
        echo "out.bk directory exists"
        rm -rf out.bk
    fi
    mv out out.bk
fi
mkdir -p out

cargo build --release

for input in `ls ../generator/in`;
do
    echo "Running test $input"
    cargo run --bin solver --release < ../generator/in/$input > out/$input
    echo "Validating $input"
    cargo run --bin validator ../generator/in/$input ./out/$input
    pushd ../generator
    score=$(cargo run --bin score ./in/$input ../kenkoooo2/out/$input)
    echo "$score" >> ../kenkoooo2/out/$input
    echo "$input: $score" >> ../kenkoooo2/out/summary.txt
    popd
done
