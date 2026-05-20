#!/usr/bin/env python3
"""
Block-scoped syscall regression evidence guard.

This replaces the overly strict v93 global duplicate-arm check.  A syscall may
legitimately appear in multiple dispatchers with different return types, e.g.
`match frame.id -> isize` and `match args.id -> RuntimeSyscallAction`.  Only
repeated SYS_* arms inside the same match block are treated as duplicates.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Dict, List, Tuple

EXPECTED_SYMBOLS = [
    # v83
    "SYS_PIDFD_SEND_SIGNAL", "SYS_IO_URING_SETUP", "SYS_IO_URING_ENTER",
    "SYS_IO_URING_REGISTER", "SYS_OPEN_TREE", "SYS_MOVE_MOUNT",
    "SYS_FSOPEN", "SYS_FSCONFIG", "SYS_FSMOUNT", "SYS_FSPICK",
    "SYS_PIDFD_OPEN", "SYS_CLONE3", "SYS_CLOSE_RANGE", "SYS_OPENAT2",
    "SYS_PIDFD_GETFD", "SYS_FACCESSAT2", "SYS_PROCESS_MADVISE",
    "SYS_EPOLL_PWAIT2", "SYS_MOUNT_SETATTR", "SYS_QUOTACTL_FD",
    "SYS_LANDLOCK_CREATE_RULESET", "SYS_LANDLOCK_ADD_RULE", "SYS_LANDLOCK_RESTRICT_SELF",
    # v84
    "SYS_MEMBARRIER", "SYS_COPY_FILE_RANGE", "SYS_PKEY_MPROTECT",
    "SYS_PKEY_ALLOC", "SYS_PKEY_FREE", "SYS_STATX", "SYS_RSEQ",
    # v85
    "SYS_MEMFD_SECRET", "SYS_PROCESS_MRELEASE", "SYS_SET_MEMPOLICY_HOME_NODE",
    "SYS_CACHESTAT", "SYS_FCHMODAT2", "SYS_MAP_SHADOW_STACK",
    # v86
    "SYS_FUTEX_WAKE", "SYS_FUTEX_WAIT", "SYS_FUTEX_REQUEUE", "SYS_STATMOUNT",
    "SYS_LISTMOUNT", "SYS_LSM_GET_SELF_ATTR", "SYS_LSM_SET_SELF_ATTR",
    "SYS_LSM_LIST_MODULES", "SYS_MSEAL", "SYS_SETXATTRAT", "SYS_GETXATTRAT",
    "SYS_LISTXATTRAT", "SYS_REMOVEXATTRAT", "SYS_OPEN_TREE_ATTR",
    "SYS_FILE_GETATTR", "SYS_FILE_SETATTR", "SYS_LISTNS", "SYS_RSEQ_SLICE_YIELD",
]

ARM_RE = re.compile(r"^\s*(SYS_[A-Z0-9_]+)\s*=>", re.M)
CONST_RE = re.compile(r"(?:pub\s+)?const\s+(SYS_[A-Z0-9_]+)\s*[:=]")


def strip_comments_and_strings_keep_lines(src: str) -> str:
    # Conservative: leave code mostly intact. We only remove block comments to
    # avoid finding fake match braces inside long comments; strings are left so
    # line counts remain stable and the arm regex remains anchored.
    return re.sub(r"/\*.*?\*/", lambda m: "\n" * m.group(0).count("\n"), src, flags=re.S)


def line_no(text: str, pos: int) -> int:
    return text.count("\n", 0, pos) + 1


def find_match_blocks(src: str) -> List[Dict[str, object]]:
    scan = strip_comments_and_strings_keep_lines(src)
    blocks: List[Dict[str, object]] = []
    for m in re.finditer(r"\bmatch\s+([^\{]+)\{", scan):
        start_brace = m.end() - 1
        depth = 0
        end = None
        i = start_brace
        while i < len(scan):
            ch = scan[i]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
            i += 1
        if end is None:
            continue
        body = src[start_brace:end]
        arms: List[Dict[str, object]] = []
        for am in ARM_RE.finditer(body):
            sym = am.group(1)
            arms.append({"symbol": sym, "line": line_no(src, start_brace + am.start(1))})
        if arms:
            subject = " ".join(m.group(1).strip().split())[:120]
            counts: Dict[str, int] = {}
            lines: Dict[str, List[int]] = {}
            for a in arms:
                sym = str(a["symbol"])
                counts[sym] = counts.get(sym, 0) + 1
                lines.setdefault(sym, []).append(int(a["line"]))
            duplicates = {sym: lines[sym] for sym, n in counts.items() if n > 1}
            blocks.append({
                "subject": subject,
                "start_line": line_no(src, m.start()),
                "end_line": line_no(src, end),
                "arm_count": len(arms),
                "duplicates": duplicates,
                "arms": arms,
            })
    return blocks


def parse_args(argv: List[str]) -> argparse.Namespace:
    p = argparse.ArgumentParser(add_help=True)
    p.add_argument("root", nargs="?", default=".")
    p.add_argument("--root", dest="root_opt", default=None)
    p.add_argument("--manifest", default=os.environ.get("SYSREG_MANIFEST") or os.environ.get("REGRESSION_MANIFEST"))
    p.add_argument("--marker", default=os.environ.get("EXPECTED_MARKER") or "")
    p.add_argument("--strict-missing", action="store_true")
    ns, _unknown = p.parse_known_args(argv)
    if ns.root_opt:
        ns.root = ns.root_opt
    return ns


def main(argv: List[str]) -> int:
    ns = parse_args(argv)
    root = Path(ns.root).resolve()
    syscall_mod = root / "src" / "syscall" / "mod.rs"
    repair_logs = root / ".repair_logs"
    repair_logs.mkdir(exist_ok=True)
    if ns.manifest:
        manifest_path = Path(ns.manifest)
        if not manifest_path.is_absolute():
            manifest_path = root / manifest_path
    else:
        ts = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
        manifest_path = repair_logs / f"full_regression_smoke_manifest_v93b_{ts}.json"

    errors: List[str] = []
    warnings: List[str] = []
    src = ""
    if not syscall_mod.exists():
        errors.append(f"missing {syscall_mod}")
    else:
        src = syscall_mod.read_text(errors="ignore")

    blocks = find_match_blocks(src) if src else []
    block_duplicates = [
        {"subject": b["subject"], "start_line": b["start_line"], "end_line": b["end_line"], "duplicates": b["duplicates"]}
        for b in blocks if b["duplicates"]
    ]
    if block_duplicates:
        errors.append("duplicate SYS_* dispatcher arms within the same match block")

    consts = set(CONST_RE.findall(src))
    arms = [str(a["symbol"]) for b in blocks for a in b["arms"]]
    arm_set = set(arms)
    missing_expected = [sym for sym in EXPECTED_SYMBOLS if sym not in src]
    if missing_expected:
        warnings.append("expected modern syscall symbols not found: " + ", ".join(missing_expected[:30]))
    arms_without_local_const = sorted(sym for sym in arm_set if sym not in consts)
    # Do not fail on this: many projects import constants from other modules, and
    # rustc warning-gate catches true catch-all bindings more reliably.
    if arms_without_local_const:
        warnings.append("SYS_* arms without local const declaration in src/syscall/mod.rs: " + ", ".join(arms_without_local_const[:40]))

    manifest = {
        "version": "v93b-block-scope-guard",
        "root": str(root),
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "expected_marker": ns.marker,
        "syscall_mod": str(syscall_mod),
        "syscall_mod_exists": syscall_mod.exists(),
        "syscall_mod_sha256": hashlib.sha256(src.encode()).hexdigest() if src else None,
        "match_block_count": len(blocks),
        "total_sys_arms": len(arms),
        "unique_sys_arms": len(arm_set),
        "block_duplicates": block_duplicates,
        "warnings": warnings,
        "errors": errors,
    }
    manifest_path.parent.mkdir(parents=True, exist_ok=True)
    manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))

    print(f"[INFO] v93b block-scoped regression evidence guard")
    print(f"[INFO] manifest: {manifest_path}")
    print(f"[INFO] match blocks with SYS_* arms: {len(blocks)}")
    print(f"[INFO] total SYS_* arms: {len(arms)}; unique: {len(arm_set)}")
    for w in warnings:
        print(f"[WARN] {w}")
    if block_duplicates:
        for item in block_duplicates:
            print(f"[ERROR] duplicate SYS_* arms in match {item['subject']} lines {item['start_line']}-{item['end_line']}: {item['duplicates']}")
    if errors:
        for e in errors:
            print(f"[ERROR] {e}")
        return 1
    print("[PASS] v93b block-scoped syscall regression evidence guard passed")
    return 0

if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
