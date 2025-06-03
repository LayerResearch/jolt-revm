#!/bin/bash

for file in guest/src/utils/*.rs; do
    filename=$(basename "$file")
    if [ "$filename" = "mod.rs" ]; then
        continue
    fi
    tester="${filename%.rs}"
    echo "=== Running for TESTER=$tester ==="
    make build-spike TESTER=$tester || exit 1
    make run-spike TESTER=$tester || exit 1
done
