#!/usr/bin/env bash
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT" || exit 1
mkdir -p .repair_logs
TS="${REGRESSION_TS:-$(date +%Y%m%d_%H%M%S)}"
MANIFEST="${REGRESSION_MANIFEST:-$ROOT/.repair_logs/full_regression_smoke_manifest_v93b_${TS}.json}"
MARKER="${EXPECTED_MARKER:-hello from external init.elf v93b syscall write}"
echo "[INFO] running full regression smoke guard from $ROOT"
EXPECTED_MARKER="$MARKER" python3 tools/regression_evidence_v93.py "$ROOT" --manifest "$MANIFEST" --marker "$MARKER"
