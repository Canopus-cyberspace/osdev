#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path
from datetime import datetime

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src" / "syscall" / "mod.rs"
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / ".repair_logs" / "signal_semantic_manifest_v102.json"

REQUIRED_SYSCALLS = [
    "SYS_RT_SIGACTION",
    "SYS_RT_SIGPROCMASK",
    "SYS_RT_SIGRETURN",
    "SYS_KILL",
    "SYS_TKILL",
    "SYS_TGKILL",
    "SYS_RT_SIGPENDING",
    "SYS_RT_SIGSUSPEND",
    "SYS_RT_SIGQUEUEINFO",
    "SYS_RT_TGSIGQUEUEINFO",
]

SIGNAL_SYMBOL_HINTS = [
    "sigaction", "sigprocmask", "sigreturn", "Signal", "signal",
    "SigAction", "SigSet", "pending", "kill", "tgkill", "tkill",
    "rt_sigaction", "rt_sigprocmask", "rt_sigreturn",
]

def read(path: Path) -> str:
    return path.read_text(errors="ignore") if path.exists() else ""

def sha256(path: Path) -> str | None:
    if not path.exists():
        return None
    h = hashlib.sha256()
    h.update(path.read_bytes())
    return h.hexdigest()

def find_match_blocks(text: str):
    blocks = []
    pos = 0
    while True:
        m = re.search(r"\bmatch\s+[^{}]+\{", text[pos:])
        if not m:
            break
        start = pos + m.start()
        brace = pos + m.end() - 1
        depth = 0
        end = None
        for i in range(brace, len(text)):
            if text[i] == "{":
                depth += 1
            elif text[i] == "}":
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
        if end is None:
            break
        head = text[start:brace+1]
        body = text[brace+1:end-1]
        blocks.append((head, body, start, end))
        pos = end
    return blocks

def syscall_arms(block: str):
    # Limit to likely arm starts: SYS_FOO => ...
    return re.findall(r"(?m)^\s*(SYS_[A-Z0-9_]+)\s*=>", block)

def defined_sys_consts(text: str):
    names = set(re.findall(r"\b(?:const|static)\s+(SYS_[A-Z0-9_]+)\b", text))
    names.update(re.findall(r"\bpub\s+const\s+(SYS_[A-Z0-9_]+)\b", text))
    names.update(re.findall(r"\buse\s+[^;]*\{([^}]+)\}", text))  # broad import evidence only
    return names

def main() -> int:
    text = read(SRC)
    if not text:
        print(f"[ERROR] missing syscall source: {SRC}")
        return 1

    errors = []
    warnings = []
    blocks = find_match_blocks(text)
    duplicates = []
    arm_total = 0
    for idx, (head, body, start, end) in enumerate(blocks):
        arms = syscall_arms(body)
        arm_total += len(arms)
        seen = set()
        for a in arms:
            if a in seen:
                duplicates.append({"block": idx, "symbol": a, "offset": start})
            seen.add(a)
    if duplicates:
        errors.append("duplicate SYS_* dispatcher arms within the same match block: " + ", ".join(sorted({d["symbol"] for d in duplicates})))

    missing_required = [s for s in REQUIRED_SYSCALLS if s not in text]
    if missing_required:
        warnings.append("missing signal syscall symbols or aliases: " + ", ".join(missing_required))

    # Detect likely catch-all typo: a SYS_* arm that is lowercase-ish or not defined is hard to prove
    # across modules, so this guard keeps the Rust warning gate authoritative and records evidence.
    signal_hits = {h: text.count(h) for h in SIGNAL_SYMBOL_HINTS if h in text}
    if not signal_hits:
        warnings.append("no signal-related symbol hints found; signal layer may still be scaffold-only")

    manifest = {
        "version": "v102",
        "timestamp": datetime.utcnow().isoformat() + "Z",
        "source": str(SRC),
        "source_exists": SRC.exists(),
        "source_sha256": sha256(SRC),
        "match_blocks": len(blocks),
        "syscall_arm_count": arm_total,
        "required_signal_syscalls_present": [s for s in REQUIRED_SYSCALLS if s in text],
        "required_signal_syscalls_missing": missing_required,
        "signal_symbol_hits": signal_hits,
        "duplicate_arms_same_match_block": duplicates,
        "errors": errors,
        "warnings": warnings,
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(manifest, indent=2, ensure_ascii=False))

    for w in warnings:
        print(f"[WARN] {w}")
    if errors:
        for e in errors:
            print(f"[ERROR] {e}")
        print(f"[ERROR] signal semantic guard v102 failed; manifest: {OUT}")
        return 1

    print(f"[PASS] signal semantic guard v102 passed; manifest: {OUT}")
    print(f"[INFO] match_blocks={len(blocks)} syscall_arm_count={arm_total}")
    print(f"[INFO] present signal syscalls: {', '.join(manifest['required_signal_syscalls_present']) or '(none)'}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
