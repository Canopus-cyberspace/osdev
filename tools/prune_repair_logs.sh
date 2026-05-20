#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-/home/lenovo/projects/uestc-kernel}"
LOG_DIR="$ROOT/.repair_logs"
KEEP_NON_EVIDENCE="${KEEP_REPAIR_RUNS:-3}"
MAX_EVIDENCE_MB="${MAX_EVIDENCE_MB:-500}"
MAX_ARTIFACT_MB="${MAX_ARTIFACT_MB:-50}"

mkdir -p "$LOG_DIR"

echo "[repair-log-policy] root=$ROOT"
echo "[repair-log-policy] before:"
du -h --max-depth=1 "$LOG_DIR" 2>/dev/null | sort -h || true

artifact_summary="$LOG_DIR/large_artifact_manifest_$(date +%Y%m%d_%H%M%S).txt"
tmp_summary="$(mktemp)"

# 1. Delete forbidden binary/image artifacts from .repair_logs.
find "$LOG_DIR" -type f \( \
  -name "*.img" -o \
  -name "*.img.gz" -o \
  -name "*.xz" -o \
  -name "*.elf" -o \
  -name "*.bin" -o \
  -name "kernel-rv" -o \
  -name "kernel-la" \
\) -print0 2>/dev/null | while IFS= read -r -d '' f; do
  size="$(stat -c '%s' "$f" 2>/dev/null || echo 0)"
  sha="$(sha256sum "$f" 2>/dev/null | awk '{print $1}' || echo unknown)"
  typ="$(file -b "$f" 2>/dev/null || echo unknown)"
  printf 'DELETE_FORBIDDEN_ARTIFACT path=%s size=%s sha256=%s type=%s\n' "$f" "$size" "$sha" "$typ" >> "$tmp_summary"
  rm -f "$f"
done

# 2. Delete large non-evidence files after recording metadata.
find "$LOG_DIR" -type f -size +"${MAX_ARTIFACT_MB}"M ! -path "*evidence*" -print0 2>/dev/null | while IFS= read -r -d '' f; do
  size="$(stat -c '%s' "$f" 2>/dev/null || echo 0)"
  sha="$(sha256sum "$f" 2>/dev/null | awk '{print $1}' || echo unknown)"
  typ="$(file -b "$f" 2>/dev/null || echo unknown)"
  printf 'DELETE_LARGE_ARTIFACT path=%s size=%s sha256=%s type=%s\n' "$f" "$size" "$sha" "$typ" >> "$tmp_summary"
  rm -f "$f"
done

# 3. Gzip text evidence/logs larger than 1MB.
find "$LOG_DIR" -type f \( \
  -name "*.log" -o \
  -name "*.txt" -o \
  -name "*.json" -o \
  -name "*.md" \
\) -size +1M ! -name "*.gz" -print0 2>/dev/null | while IFS= read -r -d '' f; do
  gzip -f "$f" || true
done

# 4. Evidence directories are preserved unless oversized.
find "$LOG_DIR" -mindepth 1 -maxdepth 1 -type d -name "*evidence*" -print0 2>/dev/null | while IFS= read -r -d '' d; do
  kb="$(du -sk "$d" 2>/dev/null | awk '{print $1}' || echo 0)"
  mb=$((kb / 1024))
  if [ "$mb" -gt "$MAX_EVIDENCE_MB" ]; then
    printf 'DELETE_OVERSIZED_EVIDENCE_DIR path=%s size_mb=%s limit_mb=%s\n' "$d" "$mb" "$MAX_EVIDENCE_MB" >> "$tmp_summary"
    rm -rf "$d"
  fi
done

# 5. Remove oversized non-evidence run directories over 500MB first.
find "$LOG_DIR" -mindepth 1 -maxdepth 1 -type d ! -name "*evidence*" -print0 2>/dev/null | while IFS= read -r -d '' d; do
  kb="$(du -sk "$d" 2>/dev/null | awk '{print $1}' || echo 0)"
  mb=$((kb / 1024))
  if [ "$mb" -gt 500 ]; then
    printf 'DELETE_LARGE_NON_EVIDENCE_DIR path=%s size_mb=%s\n' "$d" "$mb" >> "$tmp_summary"
    rm -rf "$d"
  fi
done

# 6. Keep only latest KEEP_NON_EVIDENCE non-evidence run directories.
find "$LOG_DIR" -mindepth 1 -maxdepth 1 -type d ! -name "*evidence*" -printf '%T@ %p\n' 2>/dev/null \
  | sort -nr \
  | awk -v keep="$KEEP_NON_EVIDENCE" 'NR>keep {print $2}' \
  | while IFS= read -r d; do
      [ -n "$d" ] || continue
      printf 'DELETE_OLD_NON_EVIDENCE_DIR path=%s\n' "$d" >> "$tmp_summary"
      rm -rf "$d"
    done

if [ -s "$tmp_summary" ]; then
  {
    echo "# Large artifact cleanup manifest"
    echo "timestamp=$(date -Is)"
    echo "root=$ROOT"
    echo
    cat "$tmp_summary"
  } >> "$artifact_summary"
  echo "[repair-log-policy] wrote manifest: $artifact_summary"
else
  rm -f "$artifact_summary"
fi
rm -f "$tmp_summary"

echo "[repair-log-policy] after:"
du -h --max-depth=1 "$LOG_DIR" 2>/dev/null | sort -h || true
