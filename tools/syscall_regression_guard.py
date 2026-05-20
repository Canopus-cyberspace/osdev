#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
import time
from pathlib import Path

ROOT = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path.cwd().resolve()
OUT = Path(sys.argv[2]).resolve() if len(sys.argv) > 2 else None
MOD = ROOT / "src/syscall/mod.rs"
MARKER = sys.argv[3] if len(sys.argv) > 3 else ""

EXPECTED_RECENT = [
    "SYS_IO_URING_SETUP", "SYS_IO_URING_ENTER", "SYS_IO_URING_REGISTER",
    "SYS_OPEN_TREE", "SYS_MOVE_MOUNT", "SYS_FSOPEN", "SYS_FSCONFIG", "SYS_FSMOUNT", "SYS_FSPICK",
    "SYS_PIDFD_OPEN", "SYS_CLONE3", "SYS_CLOSE_RANGE", "SYS_OPENAT2", "SYS_PIDFD_GETFD",
    "SYS_FACCESSAT2", "SYS_PROCESS_MADVISE", "SYS_EPOLL_PWAIT2", "SYS_MEMFD_SECRET", "SYS_PROCESS_MRELEASE",
    "SYS_FUTEX_WAITV", "SYS_SET_MEMPOLICY_HOME_NODE",
    "SYS_BPF", "SYS_EXECVEAT", "SYS_USERFAULTFD", "SYS_MEMBARRIER", "SYS_MLOCK2",
    "SYS_COPY_FILE_RANGE", "SYS_PREADV2", "SYS_PWRITEV2", "SYS_PKEY_MPROTECT", "SYS_PKEY_ALLOC",
    "SYS_PKEY_FREE", "SYS_STATX", "SYS_IO_PGETEVENTS", "SYS_RSEQ", "SYS_KEXEC_FILE_LOAD",
    "SYS_FUTEX_WAKE", "SYS_FUTEX_WAIT", "SYS_FUTEX_REQUEUE", "SYS_STATMOUNT", "SYS_LISTMOUNT",
    "SYS_LSM_GET_SELF_ATTR", "SYS_LSM_SET_SELF_ATTR", "SYS_LSM_LIST_MODULES", "SYS_MSEAL",
    "SYS_SETXATTRAT", "SYS_GETXATTRAT", "SYS_LISTXATTRAT", "SYS_REMOVEXATTRAT",
    "SYS_OPEN_TREE_ATTR", "SYS_FILE_GETATTR", "SYS_FILE_SETATTR", "SYS_LISTNS", "SYS_RSEQ_SLICE_YIELD",
]

def line_no(text: str, pos: int) -> int:
    return text.count("\n", 0, pos) + 1

def find_match_blocks(text: str):
    blocks = []
    for m in re.finditer(r"\bmatch\s+[^{}]+\{", text):
        open_pos = text.find("{", m.start())
        if open_pos < 0:
            continue
        depth = 0
        end = None
        i = open_pos
        in_str = None
        esc = False
        while i < len(text):
            ch = text[i]
            if in_str:
                if esc:
                    esc = False
                elif ch == "\\":
                    esc = True
                elif ch == in_str:
                    in_str = None
            else:
                if ch in ('"', "'"):
                    in_str = ch
                elif ch == "{":
                    depth += 1
                elif ch == "}":
                    depth -= 1
                    if depth == 0:
                        end = i + 1
                        break
            i += 1
        if end:
            header = text[m.start():open_pos].strip()
            blocks.append((m.start(), end, header, text[open_pos + 1:end - 1]))
    return blocks

if not MOD.exists():
    print(f"[ERROR] missing {MOD}")
    sys.exit(2)

text = MOD.read_text(encoding="utf-8", errors="replace")
const_defs = set(re.findall(r"\b(?:pub\s+)?const\s+(SYS_[A-Z0-9_]+)\b", text))
static_defs = set(re.findall(r"\b(?:pub\s+)?static\s+(SYS_[A-Z0-9_]+)\b", text))
use_blocks = re.findall(r"(?m)^\s*use\s+[^;]+;", text)
use_defs = set()
for u in use_blocks:
    use_defs.update(re.findall(r"\b(SYS_[A-Z0-9_]+)\b", u))
visible_defs = const_defs | static_defs | use_defs

all_arms = []
duplicate_arms = []
blocks = find_match_blocks(text)
for idx, (start, end, header, body) in enumerate(blocks, 1):
    arms = []
    for a in re.finditer(r"(?m)^\s*(SYS_[A-Z0-9_]+)\s*=>", body):
        name = a.group(1)
        abs_pos = start + 1 + a.start(1)
        arms.append((name, line_no(text, abs_pos)))
        all_arms.append((name, line_no(text, abs_pos), idx, header[:80]))
    seen = {}
    for name, ln in arms:
        if name in seen:
            duplicate_arms.append({"match_index": idx, "symbol": name, "first_line": seen[name], "duplicate_line": ln, "match": header[:120]})
        else:
            seen[name] = ln

missing_decl = []
for name, ln, idx, header in all_arms:
    if name not in visible_defs:
        missing_decl.append({"symbol": name, "line": ln, "match_index": idx, "match": header})

missing_recent = [name for name in EXPECTED_RECENT if name not in text]
sha256 = hashlib.sha256(text.encode("utf-8", errors="replace")).hexdigest()
report = {
    "version": "v91",
    "generated_at_epoch": int(time.time()),
    "root": str(ROOT),
    "mod_rs": str(MOD),
    "mod_rs_sha256": sha256,
    "const_count": len(const_defs),
    "visible_sys_decl_count": len(visible_defs),
    "match_arm_count": len(all_arms),
    "match_block_count": len(blocks),
    "duplicate_arms_within_match": duplicate_arms,
    "arms_without_visible_decl": missing_decl,
    "recent_expected_symbols_missing_or_renamed": missing_recent,
    "marker": MARKER,
}

if OUT:
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

print(f"[INFO] syscall regression guard v91: const_count={report['const_count']} visible_sys_decl_count={report['visible_sys_decl_count']} match_arm_count={report['match_arm_count']} match_block_count={report['match_block_count']}")
print(f"[INFO] src/syscall/mod.rs sha256={sha256}")
if missing_recent:
    print("[WARN] recent expected symbols missing or renamed: " + ", ".join(missing_recent))
else:
    print("[PASS] recent generic syscall scaffold symbols are present")
if duplicate_arms:
    print("[ERROR] duplicate SYS_* arms found inside the same match block:")
    for item in duplicate_arms:
        print(f"  - {item['symbol']} first_line={item['first_line']} duplicate_line={item['duplicate_line']} match_index={item['match_index']} match={item['match']}")
if missing_decl:
    print("[ERROR] SYS_* arms without a visible const/static/import declaration, likely catch-all pattern risk:")
    for item in missing_decl:
        print(f"  - {item['symbol']} line={item['line']} match_index={item['match_index']} match={item['match']}")
if duplicate_arms or missing_decl:
    sys.exit(2)
print("[PASS] syscall regression guard v91 found no duplicate match arms or undeclared SYS_* arm symbols")
