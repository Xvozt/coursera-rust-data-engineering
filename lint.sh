#!/usr/bin/env bash
## Lint all code directories in the repostitory using cargo clippy.

for DIR in */; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo clippy --all-targets --all-features -- -D warnings)
done

echo "Lint complete."