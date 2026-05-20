#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
import time
from pathlib import Path
from typing import Any

ROOT = Path.cwd()
VERSION = "v94"
EXPECTED_MARKER = "hello from external init.elf v94 syscall write"

IMPORTANT_SYMBOLS = [
    "SYS_PIDFD_OPEN",
    "SYS_PIDFD_SEND_SIGNAL",
    "SYS_IO_URING_SETUP",
    "SYS_IO_URING_ENTER",
    "SYS_IO_URING_REGISTER",
    "SYS_OPENAT2",
    "SYS_CLOSE_RANGE",
    "SYS_LANDLOCK_CREATE_RULESET",
    "SYS_LANDLOCK_ADD_RULE",
    "SYS_LANDLOCK_RESTRICT_SELF",
    "SYS_FSPICK",
    "SYS_PIDFD_GETFD",
    "SYS_FACCESSAT2",
    "SYS_PROCESS_MADVISE",
    "SYS_EPOLL_PWAIT2",
    "SYS_MOUNT_SETATTR",
    "SYS_QUOTACTL_FD",
    "SYS_MEMFD_SECRET",
    "SYS_PROCESS_MRELEASE",
    "SYS_SET_MEMPOLICY_HOME_NODE",
    "SYS_FUTEX_WAKE",
    "SYS_FUTEX_WAIT",
    "SYS_FUTEX_REQUEUE",
    "SYS_STATMOUNT",
    "SYS_LISTMOUNT",
    "SYS_LSM_GET_SELF_ATTR",
    "SYS_LSM_SET_SELF_ATTR",
    "SYS_LSM_LIST_MODULES",
    "SYS_MSEAL",
    "SYS_SETXATTRAT",
    "SYS_GETXATTRAT",
    "SYS_LISTXATTRAT",
    "SYS_REMOVEXATTRAT",
    "SYS_LISTNS",
    "SYS_RSEQ_SLICE_YIELD",
]

FILES_OF_INTEREST = [
    "src/syscall/mod.rs",
    "src/trap/mod.rs",
    "src/task/mod.rs",
    "src/mm/mod.rs",
    "user/build_init_elf.py",
    "user/init.elf",
    "build.rs",
    "Cargo.toml",
    "tools/syscall_regression_guard.py",
    "tools/run_syscall_regression_suite.sh",
    "tools/run_full_regression_smoke.sh",
]

def sha256_file(path: Path) -> str | None:
    if not path.exists() or not path.is_file():
        return None
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()

def file_info(rel: str) -> dict[str, Any]:
    p = ROOT / rel
    info: dict[str, Any] = {"path": rel, "exists": p.exists()}
    if p.exists() and p.is_file():
        st = p.stat()
        info.update({
            "size": st.st_size,
            "mtime": int(st.st_mtime),
            "sha256": sha256_file(p),
        })
    return info

def strip_comments_and_strings(src: str) -> str:
    # A conservative scanner; keeps enough syntax for match-arm discovery.
    src = re.sub(r"//.*", "", src)
    src = re.sub(r"/\*.*?\*/", "", src, flags=re.S)
    src = re.sub(r'"(?:\\.|[^"\\])*"', '""', src)
    return src

def find_match_blocks(src: str) -> list[dict[str, Any]]:
    blocks: list[dict[str, Any]] = []
    for m in re.finditer(r"\bmatch\s+([^{]+)\{", src):
        selector = " ".join(m.group(1).strip().split())
        start = m.start()
        brace = src.find("{", m.start())
        if brace < 0:
            continue
        depth = 0
        end = None
        for i in range(brace, len(src)):
            ch = src[i]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
        if end is None:
            continue
        body = src[brace + 1:end - 1]
        arms = re.findall(r"(?m)^\s*(SYS_[A-Z0-9_]+)\s*=>", body)
        blocks.append({
            "selector": selector,
            "start": start,
            "end": end,
            "arms": arms,
        })
    return blocks

def analyze_syscall_mod() -> dict[str, Any]:
    p = ROOT / "src/syscall/mod.rs"
    result: dict[str, Any] = {
        "path": "src/syscall/mod.rs",
        "exists": p.exists(),
        "errors": [],
        "warnings": [],
    }
    if not p.exists():
        result["errors"].append("src/syscall/mod.rs missing")
        return result

    src_raw = p.read_text(encoding="utf-8", errors="ignore")
    src = strip_comments_and_strings(src_raw)
    consts = set(re.findall(r"\b(?:pub\s+)?(?:const|static)\s+(SYS_[A-Z0-9_]+)\b", src))
    # Also accept imported/re-exported symbols written as plain identifiers elsewhere.
    all_symbol_mentions = set(re.findall(r"\bSYS_[A-Z0-9_]+\b", src))
    blocks = find_match_blocks(src)

    duplicate_errors: list[str] = []
    missing_const_errors: list[str] = []
    block_summaries = []

    for idx, block in enumerate(blocks, start=1):
        seen: dict[str, int] = {}
        dups = []
        for arm in block["arms"]:
            seen[arm] = seen.get(arm, 0) + 1
            if seen[arm] == 2:
                dups.append(arm)
        if dups:
            duplicate_errors.append(f"match block #{idx} ({block['selector']}): {', '.join(sorted(dups))}")
        for arm in block["arms"]:
            if arm not in consts and arm not in all_symbol_mentions:
                missing_const_errors.append(f"match block #{idx} ({block['selector']}): {arm}")
        block_summaries.append({
            "index": idx,
            "selector": block["selector"],
            "arm_count": len(block["arms"]),
            "unique_arm_count": len(set(block["arms"])),
            "duplicates": sorted(dups),
        })

    missing_important = [s for s in IMPORTANT_SYMBOLS if s not in all_symbol_mentions]
    if duplicate_errors:
        result["errors"].append("duplicate SYS_* dispatcher arms inside a single match block: " + " | ".join(duplicate_errors))
    if missing_const_errors:
        result["errors"].append("SYS_* arm references without visible symbol mention: " + " | ".join(missing_const_errors))
    if missing_important:
        result["warnings"].append("important modern syscall symbols not observed: " + ", ".join(missing_important))

    result.update({
        "const_count": len(consts),
        "sys_symbol_mention_count": len(all_symbol_mentions),
        "match_block_count": len(blocks),
        "match_blocks": block_summaries,
        "important_symbols_observed": sorted([s for s in IMPORTANT_SYMBOLS if s in all_symbol_mentions]),
        "important_symbols_missing": missing_important,
        "marker_in_syscall_mod": EXPECTED_MARKER in src_raw,
    })
    return result

def latest_logs() -> list[dict[str, Any]]:
    logdir = ROOT / ".repair_logs"
    if not logdir.exists():
        return []
    items = sorted(
        [p for p in logdir.iterdir() if p.is_file()],
        key=lambda p: p.stat().st_mtime,
        reverse=True,
    )[:40]
    out = []
    for p in items:
        out.append({
            "path": str(p.relative_to(ROOT)),
            "size": p.stat().st_size,
            "mtime": int(p.stat().st_mtime),
            "sha256": sha256_file(p),
        })
    return out

def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--manifest", required=True)
    ap.add_argument("--guard-log", required=True)
    args = ap.parse_args()

    analysis = analyze_syscall_mod()
    manifest: dict[str, Any] = {
        "version": VERSION,
        "created_at_unix": int(time.time()),
        "project": str(ROOT),
        "expected_marker": EXPECTED_MARKER,
        "files": [file_info(rel) for rel in FILES_OF_INTEREST],
        "syscall_analysis": analysis,
        "latest_logs": latest_logs(),
    }

    manifest_path = Path(args.manifest)
    manifest_path.parent.mkdir(parents=True, exist_ok=True)
    manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True), encoding="utf-8")

    guard_lines = [
        f"[INFO] regression baseline snapshot guard {VERSION}",
        f"[INFO] project: {ROOT}",
        f"[INFO] expected marker: {EXPECTED_MARKER}",
        f"[INFO] manifest: {manifest_path}",
        f"[INFO] syscall const count: {analysis.get('const_count')}",
        f"[INFO] syscall symbol mention count: {analysis.get('sys_symbol_mention_count')}",
        f"[INFO] match block count: {analysis.get('match_block_count')}",
    ]

    for block in analysis.get("match_blocks", []):
        guard_lines.append(
            f"[INFO] match block #{block['index']} selector={block['selector']} arms={block['arm_count']} unique={block['unique_arm_count']}"
        )

    for warning in analysis.get("warnings", []):
        guard_lines.append(f"[WARN] {warning}")

    if analysis.get("errors"):
        for err in analysis["errors"]:
            guard_lines.append(f"[ERROR] {err}")
        guard_lines.append("[ERROR] regression baseline snapshot guard v94 failed")
        Path(args.guard_log).write_text("\n".join(guard_lines) + "\n", encoding="utf-8")
        print("\n".join(guard_lines))
        return 1

    guard_lines.append("[PASS] regression baseline snapshot guard v94 passed")
    Path(args.guard_log).write_text("\n".join(guard_lines) + "\n", encoding="utf-8")
    print("\n".join(guard_lines))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
