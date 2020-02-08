#!/bin/bash

features=$(cat Cargo.toml| grep -A 10000 "\[features\]" | grep "stm32.* = \[\]" | cut -d " " -f1)

set -ex
for feature in $features; do
    cargo check --features $feature
done
