#!/usr/bin/env bash
set -Eeuo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p .repair_logs
STAMP="${1:-$(date +%Y%m%d_%H%M%S)}"
GUARD_LOG=".repair_logs/fd_vfs_semantic_guard_v95_${STAMP}.log"
MANIFEST=".repair_logs/fd_vfs_semantic_manifest_v95_${STAMP}.json"
python3 tools/fd_vfs_semantic_guard_v95.py --manifest "$MANIFEST" --guard-log "$GUARD_LOG"
echo "[PASS] v95 FD/VFS semantic manifest: $MANIFEST"
