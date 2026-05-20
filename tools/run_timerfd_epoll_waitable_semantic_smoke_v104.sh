#!/usr/bin/env bash
set -Eeuo pipefail
PROJECT="${1:-$(pwd)}"
LOG="${2:-$PROJECT/.repair_logs/timerfd_epoll_waitable_semantic_guard_v104_manual.log}"
MANIFEST="${3:-$PROJECT/.repair_logs/timerfd_epoll_waitable_semantic_manifest_v104_manual.json}"
cd "$PROJECT"
python3 tools/timerfd_epoll_waitable_semantic_guard_v104.py --project "$PROJECT" --log "$LOG" --manifest "$MANIFEST"
