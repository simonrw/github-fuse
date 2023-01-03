#!/usr/bin/env bash

set -euo pipefail

MOUNT_DIR=/tmp/github-fuse-mnt
umount $MOUNT_DIR 2>/dev/null || true
cargo run -- -m "$MOUNT_DIR"
