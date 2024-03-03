#!/bin/bash

set -euxo pipefail

cd "$(dirname "$0")"

input="in/$1.txt"
output="out/$1.txt"

cargo run --release < ../generator/"$input" > "$output"

cd ../generator
cargo run -r --bin score "$input"  "../garasubo/$output"