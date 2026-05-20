#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import os
import re
from pathlib import Path
from datetime import datetime, timezone

ROOT = Path.cwd()
SRC = ROOT / "src" / "syscall" / "mod.rs"
LOG = Path(os.environ.get("V106_GUARD_LOG", ROOT / ".repair_logs" / "vfs_user_behavior_guard_v106.log"))
MANIFEST = Path(os.environ.get("V106_MANIFEST", ROOT / ".repair_logs" / "vfs_user_behavior_manifest_v106.json"))
MARKER = os.environ.get("V106_MARKER", "hello from external init.elf v106 syscall write")

REQUIRED_SYSCALLS = [
    "SYS_OPENAT", "SYS_CLOSE", "SYS_READ", "SYS_WRITE",
    "SYS_FSTAT", "SYS_LSEEK", "SYS_GETDENTS64",
]
RELATED_SYSCALLS = [
    "SYS_STATX", "SYS_NEWFSTATAT", "SYS_FCNTL", "SYS_FACCESSAT",
    "SYS_READV", "SYS_WRITEV", "SYS_PREAD64", "SYS_PWRITE64",
]
SEMANTIC_HINTS = {
    "fd_table_or_fd": [r"fd[_A-Za-z0-9]*table", r"FileDescriptor", r"alloc_fd", r"close_fd", r"\bfd\b"],
    "open_path": [r"openat", r"OpenAt", r"path", r"user_path"],
    "read_write_ops": [r"sys_read", r"sys_write", r"\bread\(", r"\bwrite\(", r"Read", r"Write"],
    "seek_offset": [r"lseek", r"offset", r"Seek"],
    "stat_metadata": [r"fstat", r"statx", r"metadata", r"Stat"],
    "dirent_getdents": [r"getdents64", r"dirent", r"Dirent", r"getdents"],
    "user_copy": [r"copy_from_user", r"copy_to_user", r"user[_A-Za-z0-9]*ptr", r"EFAULT", r"User"],
}

def log(msg: str) -> None:
    LOG.parent.mkdir(parents=True, exist_ok=True)
    with LOG.open("a", encoding="utf-8") as f:
        f.write(msg + "\n")
    print(msg)

def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8", errors="ignore")
    except FileNotFoundError:
        return ""

def find_matching_brace(text: str, open_idx: int) -> int:
    depth = 0
    i = open_idx
    in_line = False
    in_block = False
    in_str = False
    esc = False
    while i < len(text):
        ch = text[i]
        nxt = text[i+1] if i + 1 < len(text) else ""
        if in_line:
            if ch == "\n":
                in_line = False
            i += 1
            continue
        if in_block:
            if ch == "*" and nxt == "/":
                in_block = False
                i += 2
                continue
            i += 1
            continue
        if in_str:
            if esc:
                esc = False
            elif ch == "\\":
                esc = True
            elif ch == '"':
                in_str = False
            i += 1
            continue
        if ch == "/" and nxt == "/":
            in_line = True
            i += 2
            continue
        if ch == "/" and nxt == "*":
            in_block = True
            i += 2
            continue
        if ch == '"':
            in_str = True
            i += 1
            continue
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1

def match_blocks(text: str):
    for m in re.finditer(r"\bmatch\s+[^{}]+{", text):
        open_idx = text.find("{", m.start())
        close_idx = find_matching_brace(text, open_idx)
        if close_idx > open_idx:
            yield (m.start(), close_idx, text[m.start():close_idx+1])

def duplicate_arms_per_block(text: str):
    issues = []
    for start, close, block in match_blocks(text):
        arms = re.findall(r"(?m)^\s*(SYS_[A-Z0-9_]+)\s*=>", block)
        seen = set()
        dup = []
        for a in arms:
            if a in seen and a not in dup:
                dup.append(a)
            seen.add(a)
        if dup:
            line = text.count("\n", 0, start) + 1
            issues.append({"line": line, "duplicates": dup})
    return issues

def main() -> int:
    LOG.parent.mkdir(parents=True, exist_ok=True)
    LOG.write_text("", encoding="utf-8")
    text = read_text(SRC)
    if not text:
        log(f"[ERROR] missing syscall source: {SRC}")
        return 1

    manifest = {
        "version": "v106",
        "created_at": datetime.now(timezone.utc).isoformat(),
        "project": str(ROOT),
        "marker": MARKER,
        "files": {},
        "required_syscalls": {},
        "related_syscalls": {},
        "semantic_hints": {},
        "duplicate_match_arm_issues": [],
        "debts": [],
    }

    for p in [SRC, ROOT/"user"/"build_init_elf.py", ROOT/"user"/"init.elf", ROOT/"Makefile"]:
        if p.exists():
            data = p.read_bytes()
            manifest["files"][str(p.relative_to(ROOT))] = {
                "size": len(data),
                "sha256": hashlib.sha256(data).hexdigest(),
            }

    ok = True
    for sym in REQUIRED_SYSCALLS:
        present = bool(re.search(r"\b" + re.escape(sym) + r"\b", text))
        manifest["required_syscalls"][sym] = present
        if present:
            log(f"[PASS] required VFS syscall symbol present: {sym}")
        else:
            log(f"[ERROR] missing required VFS syscall symbol: {sym}")
            ok = False

    for sym in RELATED_SYSCALLS:
        present = bool(re.search(r"\b" + re.escape(sym) + r"\b", text))
        manifest["related_syscalls"][sym] = present
        if present:
            log(f"[INFO] related VFS syscall symbol present: {sym}")
        else:
            manifest["debts"].append(f"related syscall not found or intentionally absent: {sym}")

    for name, pats in SEMANTIC_HINTS.items():
        hits = []
        for pat in pats:
            if re.search(pat, text, flags=re.IGNORECASE):
                hits.append(pat)
        manifest["semantic_hints"][name] = hits
        if hits:
            log(f"[PASS] semantic hint present for {name}: {hits[:3]}")
        else:
            log(f"[WARN] semantic hint missing for {name}; recorded as implementation debt")
            manifest["debts"].append(f"semantic hint missing: {name}")

    dups = duplicate_arms_per_block(text)
    manifest["duplicate_match_arm_issues"] = dups
    if dups:
        for issue in dups:
            log(f"[ERROR] duplicate SYS_* arms inside one match block near line {issue['line']}: {', '.join(issue['duplicates'])}")
        ok = False
    else:
        log("[PASS] no duplicate SYS_* arms inside individual match blocks")

    # Detect classic catch-all risk where a missing const accidentally becomes a binding.
    suspicious = []
    for sym in REQUIRED_SYSCALLS + RELATED_SYSCALLS:
        if re.search(r"(?m)^\s*" + re.escape(sym) + r"\s*=>", text) and not re.search(r"\b(?:const|static)\s+" + re.escape(sym) + r"\b", text):
            # The project may import constants from other modules; treat as warning unless build later reports "matches any value".
            suspicious.append(sym)
    manifest["potential_imported_or_missing_consts"] = suspicious
    if suspicious:
        log("[WARN] syscall arms without local const/static declaration; build warning gate will decide catch-all risk: " + ", ".join(suspicious))
    else:
        log("[PASS] no obvious local missing-const risk for checked VFS syscall arms")

    MANIFEST.parent.mkdir(parents=True, exist_ok=True)
    MANIFEST.write_text(json.dumps(manifest, indent=2, sort_keys=True), encoding="utf-8")
    log(f"[INFO] manifest written: {MANIFEST}")

    if not ok:
        log(f"[ERROR] VFS user behavior guard v106 failed; manifest: {MANIFEST}")
        return 1
    log("[PASS] VFS user behavior semantic guard v106 passed")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
