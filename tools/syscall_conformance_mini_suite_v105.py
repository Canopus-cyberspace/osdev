#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Dict, List, Tuple


CORE_SYSCALLS = [
    "SYS_OPENAT", "SYS_CLOSE", "SYS_READ", "SYS_WRITE", "SYS_FSTAT", "SYS_LSEEK", "SYS_GETDENTS64",
    "SYS_MMAP", "SYS_MUNMAP", "SYS_MPROTECT", "SYS_BRK",
    "SYS_CLONE", "SYS_EXECVE", "SYS_WAIT4", "SYS_EXIT", "SYS_EXIT_GROUP", "SYS_GETPID", "SYS_GETPPID",
    "SYS_PIPE2", "SYS_DUP", "SYS_DUP3", "SYS_EVENTFD2", "SYS_POLL", "SYS_PPOLL",
    "SYS_EPOLL_CREATE1", "SYS_EPOLL_CTL", "SYS_EPOLL_PWAIT", "SYS_EPOLL_PWAIT2",
    "SYS_TIMERFD_CREATE", "SYS_TIMERFD_SETTIME", "SYS_TIMERFD_GETTIME",
    "SYS_FUTEX", "SYS_SET_TID_ADDRESS", "SYS_SET_ROBUST_LIST", "SYS_GET_ROBUST_LIST",
    "SYS_RT_SIGACTION", "SYS_RT_SIGPROCMASK", "SYS_RT_SIGRETURN", "SYS_KILL", "SYS_TKILL", "SYS_TGKILL",
    "SYS_SOCKET", "SYS_SOCKETPAIR", "SYS_SENDMSG", "SYS_RECVMSG", "SYS_SENDTO", "SYS_RECVFROM",
]

SEMANTIC_TERMS = {
    "fd_vfs": ["openat", "fstat", "lseek", "getdents64", "File", "fd"],
    "usercopy_iovec": ["copy_from_user", "copy_to_user", "iovec", "timespec", "EFAULT"],
    "mmap_brk": ["mmap", "munmap", "mprotect", "brk"],
    "process": ["clone", "execve", "wait4", "exit_group"],
    "waitable_fd": ["pipe", "eventfd", "poll", "epoll"],
    "futex_scheduler": ["futex", "wake", "sched_yield"],
    "signal": ["rt_sigaction", "rt_sigprocmask", "rt_sigreturn", "kill"],
    "socket_loopback": ["socketpair", "sendmsg", "recvmsg"],
    "timerfd_epoll": ["timerfd", "epoll", "ppoll"],
}


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def extract_match_blocks(text: str) -> List[Tuple[int, str]]:
    blocks: List[Tuple[int, str]] = []
    for m in re.finditer(r"\bmatch\s+[^{}]+{", text):
        start = m.start()
        brace = text.find("{", m.start())
        if brace < 0:
            continue
        depth = 0
        end = None
        for i in range(brace, len(text)):
            c = text[i]
            if c == "{":
                depth += 1
            elif c == "}":
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
        if end is not None:
            line = text.count("\n", 0, start) + 1
            blocks.append((line, text[start:end]))
    return blocks


def duplicate_arms_by_block(text: str) -> List[str]:
    errors: List[str] = []
    for line, block in extract_match_blocks(text):
        arms = re.findall(r"\b(SYS_[A-Z0-9_]+)\s*=>", block)
        seen: Dict[str, int] = {}
        for arm in arms:
            seen[arm] = seen.get(arm, 0) + 1
        dup = sorted(k for k, v in seen.items() if v > 1)
        if dup:
            errors.append(f"match block starting at line {line} has duplicate SYS_* arms: {', '.join(dup)}")
    return errors


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--project", default=".")
    ap.add_argument("--manifest", required=True)
    ap.add_argument("--guard-log", required=True)
    ap.add_argument("--marker", required=True)
    args = ap.parse_args()

    root = Path(args.project).resolve()
    guard_log = Path(args.guard_log)
    manifest_path = Path(args.manifest)

    errors: List[str] = []
    warnings: List[str] = []
    evidence: Dict[str, object] = {"version": "v105", "marker": args.marker}

    key_files = [
        "Cargo.toml",
        "build.rs",
        "src/syscall/mod.rs",
        "user/build_init_elf.py",
        "user/init.elf",
        "tools/run-qemu.sh",
    ]
    files = {}
    for rel in key_files:
        p = root / rel
        if p.exists():
            files[rel] = {"size": p.stat().st_size, "sha256": sha256(p)}
        else:
            files[rel] = None
            if rel in ("Cargo.toml", "src/syscall/mod.rs", "user/init.elf"):
                errors.append(f"required file missing: {rel}")
            else:
                warnings.append(f"optional file missing: {rel}")
    evidence["files"] = files

    syscall_path = root / "src/syscall/mod.rs"
    text = syscall_path.read_text(encoding="utf-8", errors="replace") if syscall_path.exists() else ""

    if text:
        errors.extend(duplicate_arms_by_block(text))
        consts = set(re.findall(r"\b(?:const|pub\s+const)\s+(SYS_[A-Z0-9_]+)\b", text))
        arms = set(re.findall(r"\b(SYS_[A-Z0-9_]+)\s*=>", text))
        visible = set(re.findall(r"\b(SYS_[A-Z0-9_]+)\b", text))
        missing_visible = sorted(a for a in CORE_SYSCALLS if a not in visible)
        if missing_visible:
            warnings.append("core syscall symbols not visible in src/syscall/mod.rs: " + ", ".join(missing_visible))
        missing_const_for_arms = sorted(a for a in arms if a not in consts and text.count(a) <= 2)
        if missing_const_for_arms:
            warnings.append("some arm symbols have no obvious local const; verify imports/static consts: " + ", ".join(missing_const_for_arms[:40]))
        evidence["syscall_mod"] = {
            "sys_symbol_count": len(visible),
            "match_arm_count": len(re.findall(r"\bSYS_[A-Z0-9_]+\s*=>", text)),
            "const_count": len(consts),
            "core_visible_count": sum(1 for s in CORE_SYSCALLS if s in visible),
            "match_block_count": len(extract_match_blocks(text)),
        }

    # Semantic term scan across likely source/tool files. This is a conformance mini-suite readiness guard,
    # not a full runtime conformance test yet; missing groups are warnings unless the whole source is absent.
    searchable = ""
    for sub in ["src", "tools", "user"]:
        d = root / sub
        if d.exists():
            for p in d.rglob("*"):
                if p.is_file() and p.suffix in (".rs", ".py", ".sh", ".S", ".s", ".c", ".h", ".md"):
                    try:
                        searchable += "\n" + p.read_text(encoding="utf-8", errors="replace")
                    except Exception:
                        pass
    term_report = {}
    for group, terms in SEMANTIC_TERMS.items():
        found = [t for t in terms if re.search(re.escape(t), searchable, re.IGNORECASE)]
        term_report[group] = {"found": found, "total": len(terms)}
        if not found:
            warnings.append(f"semantic group has no visible terms: {group}")
    evidence["semantic_terms"] = term_report

    init_elf = root / "user/init.elf"
    if init_elf.exists():
        data = init_elf.read_bytes()
        if args.marker.encode() not in data:
            errors.append("user/init.elf does not contain expected v105 marker")
        else:
            evidence["init_elf_marker"] = "present"

    evidence["warnings"] = warnings
    evidence["errors"] = errors

    manifest_path.write_text(json.dumps(evidence, indent=2, sort_keys=True), encoding="utf-8")
    with guard_log.open("w", encoding="utf-8") as f:
        f.write("[INFO] syscall conformance mini-suite v105 guard\n")
        f.write(f"[INFO] project: {root}\n")
        f.write(f"[INFO] marker: {args.marker}\n")
        f.write(f"[INFO] manifest: {manifest_path}\n")
        for w in warnings:
            f.write(f"[WARN] {w}\n")
        for e in errors:
            f.write(f"[ERROR] {e}\n")
        if not errors:
            f.write("[PASS] syscall conformance mini-suite v105 guard passed\n")

    print(f"[INFO] guard log: {guard_log}")
    print(f"[INFO] manifest: {manifest_path}")
    if warnings:
        print(f"[INFO] guard warnings: {len(warnings)}")
    if errors:
        for e in errors:
            print(f"[ERROR] {e}")
        return 1
    print("[PASS] syscall conformance mini-suite v105 guard passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
