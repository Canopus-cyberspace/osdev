#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
cat <<'MSG'
[build-user-init] scaffold only
[build-user-init] current v48 smoke uses checked-in user/init.elf synthetic image
[build-user-init] future step may replace this with clang/rust-lld assembly build
MSG
