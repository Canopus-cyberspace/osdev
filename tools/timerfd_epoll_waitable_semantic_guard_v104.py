#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import re
import time
from pathlib import Path

VERSION = "v104"
REQUIRED_FAMILIES = {
    "timerfd": ["timerfd", "Timerfd", "TimerFd", "SYS_TIMERFD_CREATE", "SYS_TIMERFD_SETTIME", "SYS_TIMERFD_GETTIME"],
    "epoll": ["epoll", "Epoll", "SYS_EPOLL_CREATE", "SYS_EPOLL_CREATE1", "SYS_EPOLL_CTL", "SYS_EPOLL_PWAIT", "SYS_EPOLL_PWAIT2"],
    "poll": ["SYS_POLL", "SYS_PPOLL", "poll(", "Poll", "Ppoll"],
    "time_timeout": ["timespec", "TimeSpec", "nanosleep", "clock_gettime", "timeout", "wake", "wakeup"],
    "waitable_fd": ["File", "fd", "eventfd", "pipe", "readable", "writable", "wait", "Wait"],
}

IMPORTANT_SYSCALLS = [
    "SYS_TIMERFD_CREATE",
    "SYS_TIMERFD_SETTIME",
    "SYS_TIMERFD_GETTIME",
    "SYS_EPOLL_CREATE1",
    "SYS_EPOLL_CTL",
    "SYS_EPOLL_PWAIT",
    "SYS_EPOLL_PWAIT2",
    "SYS_POLL",
    "SYS_PPOLL",
    "SYS_NANOSLEEP",
    "SYS_CLOCK_GETTIME",
]


def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return ""


def sha256(path: Path) -> str | None:
    try:
        h = hashlib.sha256()
        with path.open("rb") as f:
            for chunk in iter(lambda: f.read(1024 * 1024), b""):
                h.update(chunk)
        return h.hexdigest()
    except FileNotFoundError:
        return None


def iter_rs_files(root: Path):
    src = root / "src"
    if not src.exists():
        return []
    return sorted(src.rglob("*.rs"))


def extract_match_blocks(text: str):
    blocks = []
    i = 0
    n = len(text)
    while True:
        m = re.search(r"\bmatch\b", text[i:])
        if not m:
            break
        start = i + m.start()
        brace = text.find("{", start)
        if brace < 0:
            break
        depth = 0
        end = None
        for j in range(brace, n):
            ch = text[j]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = j + 1
                    break
        if end is None:
            break
        prefix = text[start:brace].strip()
        body = text[brace:end]
        blocks.append((start, prefix, body))
        i = end
    return blocks


def duplicate_sys_arms_by_block(path: Path):
    text = read_text(path)
    duplicates = []
    for idx, (offset, prefix, body) in enumerate(extract_match_blocks(text), start=1):
        arms = re.findall(r"\b(SYS_[A-Z0-9_]+)\s*=>", body)
        seen = set()
        dup = []
        for arm in arms:
            if arm in seen and arm not in dup:
                dup.append(arm)
            seen.add(arm)
        if dup:
            duplicates.append({
                "match_index": idx,
                "offset": offset,
                "prefix": prefix[:120],
                "duplicates": dup,
            })
    return duplicates


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--project", default=".")
    ap.add_argument("--manifest", required=True)
    ap.add_argument("--log", required=True)
    args = ap.parse_args()

    root = Path(args.project).resolve()
    log_path = Path(args.log)
    manifest_path = Path(args.manifest)
    log_path.parent.mkdir(parents=True, exist_ok=True)
    manifest_path.parent.mkdir(parents=True, exist_ok=True)

    rs_files = iter_rs_files(root)
    corpus = "\n".join(read_text(p) for p in rs_files)
    syscall_mod = root / "src" / "syscall" / "mod.rs"

    family_hits = {}
    semantic_debt = []
    for family, needles in REQUIRED_FAMILIES.items():
        hits = [needle for needle in needles if needle in corpus]
        family_hits[family] = hits
        if not hits:
            semantic_debt.append(family)

    syscall_hits = {name: (name in corpus) for name in IMPORTANT_SYSCALLS}
    duplicate_blocks = duplicate_sys_arms_by_block(syscall_mod) if syscall_mod.exists() else [{"error": "src/syscall/mod.rs missing"}]

    suspicious_catch_all = []
    if syscall_mod.exists():
        text = read_text(syscall_mod)
        # A missing const often manifests as `SYS_FOO` becoming a catch-all binding.
        # Cargo's "matches any value" warning remains the source of truth; this is an early heuristic only.
        for line_no, line in enumerate(text.splitlines(), start=1):
            if re.search(r"\bSYS_[A-Z0-9_]+\s*=>\s*SYS_[A-Z0-9_]+\b", line):
                suspicious_catch_all.append({"line": line_no, "text": line.strip()})

    manifest = {
        "version": VERSION,
        "timestamp": int(time.time()),
        "project": str(root),
        "rs_file_count": len(rs_files),
        "src_syscall_mod": {
            "exists": syscall_mod.exists(),
            "sha256": sha256(syscall_mod) if syscall_mod.exists() else None,
            "bytes": syscall_mod.stat().st_size if syscall_mod.exists() else None,
        },
        "family_hits": family_hits,
        "semantic_debt_families": semantic_debt,
        "important_syscall_hits": syscall_hits,
        "duplicate_sys_arms_by_match_block": duplicate_blocks,
        "suspicious_catch_all_heuristics": suspicious_catch_all,
    }
    manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True), encoding="utf-8")

    lines = []
    lines.append(f"[INFO] waitable FD/timer semantic guard {VERSION}")
    lines.append(f"[INFO] project: {root}")
    lines.append(f"[INFO] Rust source files scanned: {len(rs_files)}")
    for family, hits in family_hits.items():
        if hits:
            lines.append(f"[PASS] family {family}: {', '.join(hits[:8])}")
        else:
            lines.append(f"[WARN] semantic debt family has no direct symbol hit: {family}")
    missing_syscalls = [k for k, v in syscall_hits.items() if not v]
    if missing_syscalls:
        lines.append("[WARN] syscall symbols not directly observed: " + ", ".join(missing_syscalls))
    if duplicate_blocks:
        lines.append("[ERROR] duplicate SYS_* dispatcher arms found inside a single match block:")
        for item in duplicate_blocks:
            lines.append(json.dumps(item, ensure_ascii=False))
    else:
        lines.append("[PASS] no duplicate SYS_* dispatcher arms within any single match block")
    if suspicious_catch_all:
        lines.append("[ERROR] suspicious SYS_* => SYS_* catch-all-like arm(s) found:")
        for item in suspicious_catch_all:
            lines.append(f"  line {item['line']}: {item['text']}")
    else:
        lines.append("[PASS] no suspicious SYS_* => SYS_* catch-all-like arms found")

    log_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    print(f"[INFO] manifest: {manifest_path}")

    if duplicate_blocks or suspicious_catch_all:
        return 1

    # Guard is intentionally semantic/debt-oriented: missing optional symbols are logged as warnings,
    # while structural dispatcher hazards are hard failures.
    print("[PASS] timerfd/epoll/poll waitable FD semantic guard v104 passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

