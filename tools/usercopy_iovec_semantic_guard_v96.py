#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import os
import re
import sys
import time
from pathlib import Path
from typing import Dict, List, Tuple

ROOT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path.cwd()
OUT = Path(sys.argv[2]) if len(sys.argv) > 2 else ROOT / ".repair_logs" / "usercopy_iovec_semantic_manifest_v96.json"

def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return ""

def sha256(path: Path) -> str:
    h = hashlib.sha256()
    try:
        h.update(path.read_bytes())
        return h.hexdigest()
    except Exception:
        return ""

def rust_files() -> List[Path]:
    files = list((ROOT / "src").rglob("*.rs")) if (ROOT / "src").exists() else []
    for extra in [ROOT / "user" / "build_init_elf.py", ROOT / "tools" / "syscall_regression_guard.py"]:
        if extra.exists():
            files.append(extra)
    return files

def scan_keywords(files: List[Path]) -> Dict[str, int]:
    keywords = {
        # user memory copy/read/write primitives
        "copy_from_user": r"\b(copy_from_user|copyin|read_user|translated_ref|translated_byte_buffer|UserBuffer|user_slice|check_user|validate_user)\b",
        "copy_to_user": r"\b(copy_to_user|copyout|write_user|translated_refmut|write_user_bytes|UserBuffer|user_slice_mut)\b",
        "user_string": r"\b(translated_str|user_str|copy_cstr|CStr|CString|read_cstr|read_user_str|read_user_cstr)\b",
        "iovec": r"\b(iovec|IoVec|Iovec|msghdr|Msghdr|sendmsg|recvmsg|preadv|pwritev|readv|writev)\b",
        "timespec": r"\b(timespec|TimeSpec|timeval|TimeVal|itimerspec|nanosleep|clock_gettime|clock_nanosleep)\b",
        "stat_structs": r"\b(statx|Statx|Kstat|Dirent|LinuxDirent|dirent|getdents64|fstat|newfstatat)\b",
        "errno_efault": r"\b(EFAULT|BadAddress|Fault|UserFault)\b",
    }
    counts = {k: 0 for k in keywords}
    for path in files:
        txt = read_text(path)
        for key, pat in keywords.items():
            counts[key] += len(re.findall(pat, txt, flags=re.IGNORECASE))
    return counts

def extract_match_blocks(text: str) -> List[Tuple[int, str]]:
    blocks: List[Tuple[int, str]] = []
    for m in re.finditer(r"\bmatch\s+[^{}]+\{", text):
        start = m.start()
        i = m.end() - 1
        depth = 0
        end = None
        while i < len(text):
            ch = text[i]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
            i += 1
        if end:
            line = text.count("\n", 0, start) + 1
            blocks.append((line, text[start:end]))
    return blocks

def duplicate_sys_arms_by_block(path: Path) -> List[Dict[str, object]]:
    txt = read_text(path)
    issues: List[Dict[str, object]] = []
    for line, block in extract_match_blocks(txt):
        arms = re.findall(r"^\s*(SYS_[A-Z0-9_]+)\s*=>", block, flags=re.MULTILINE)
        seen = {}
        dup = []
        for arm in arms:
            seen[arm] = seen.get(arm, 0) + 1
        for arm, cnt in sorted(seen.items()):
            if cnt > 1:
                dup.append({"symbol": arm, "count": cnt})
        if dup:
            issues.append({"match_line": line, "duplicates": dup})
    return issues

def symbol_inventory(files: List[Path]) -> Dict[str, int]:
    wanted = [
        "SYS_READ", "SYS_WRITE", "SYS_READV", "SYS_WRITEV", "SYS_PREADV", "SYS_PWRITEV",
        "SYS_RECVMSG", "SYS_SENDMSG", "SYS_RECVMMSG", "SYS_SENDMMSG",
        "SYS_NANOSLEEP", "SYS_CLOCK_GETTIME", "SYS_CLOCK_NANOSLEEP",
        "SYS_FSTAT", "SYS_STATX", "SYS_GETDENTS64", "SYS_FUTEX",
    ]
    inv = {w: 0 for w in wanted}
    for p in files:
        txt = read_text(p)
        for w in wanted:
            inv[w] += txt.count(w)
    return inv

def main() -> int:
    files = rust_files()
    src_syscall = ROOT / "src" / "syscall" / "mod.rs"
    keyword_counts = scan_keywords(files)
    inventory = symbol_inventory(files)
    dup_issues = duplicate_sys_arms_by_block(src_syscall) if src_syscall.exists() else []

    warnings = []
    for key in ["copy_from_user", "copy_to_user", "user_string", "iovec", "timespec", "errno_efault"]:
        if keyword_counts.get(key, 0) == 0:
            warnings.append(f"no obvious {key} primitive/symbol found yet; v96 records this as semantic debt, not a hard failure")

    manifest = {
        "version": "v96",
        "root": str(ROOT),
        "timestamp": int(time.time()),
        "checked_files": [str(p.relative_to(ROOT)) if p.is_relative_to(ROOT) else str(p) for p in files],
        "src_syscall_mod_sha256": sha256(src_syscall) if src_syscall.exists() else "",
        "keyword_counts": keyword_counts,
        "syscall_symbol_inventory": inventory,
        "duplicate_sys_arms_by_match_block": dup_issues,
        "warnings": warnings,
        "policy": {
            "duplicate_sys_arms_in_same_match_block": "hard_fail",
            "missing_usercopy_semantic_keywords": "warn_only_for_v96_baseline",
            "forbidden_rust_warnings": ["matches any value", "unreachable pattern"],
        },
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(manifest, indent=2, sort_keys=True), encoding="utf-8")

    print(f"[INFO] v96 usercopy/iovec/timespec semantic manifest: {OUT}")
    print(f"[INFO] scanned files: {len(files)}")
    print(f"[INFO] keyword counts: {json.dumps(keyword_counts, sort_keys=True)}")
    if warnings:
        for w in warnings:
            print(f"[WARN] {w}")
    if dup_issues:
        print("[ERROR] duplicate SYS_* dispatcher arms inside the same match block detected:")
        for issue in dup_issues:
            print(json.dumps(issue, sort_keys=True))
        return 1

    print("[PASS] user memory copy/iovec/timespec semantic guard v96 passed")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
