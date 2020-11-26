#!/bin/sh

if [ $# != 1 ]; then
    echo "Usage: $(basename "$0") <day-number>" >&2
    exit 1
fi
if [ ! -d .git ]; then
    echo "must be run from root of advent-of-code repository" >&2
    exit 1
fi

name="$(printf "day%02d" "$1")"
cat Cargo.toml |grep -n ^] |cut -d ":" -f1 | xargs -I '{}' sed -i "{}i\ \ \ \ '$name'," Cargo.toml
cargo new --bin "$name"
mkdir "$name/input"
touch "$name/input/input.txt"
