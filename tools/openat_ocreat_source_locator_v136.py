#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
EXCLUDE_DIRS = {".git", "target", ".repair_logs"}
EXCLUDE_PREFIXES = (".backup_repair_", ".backup_")
REPORT_JSON = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / ".repair_logs/openat_ocreat_source_report_v136.json"
REPORT_TXT = Path(sys.argv[2]) if len(sys.argv) > 2 else ROOT / ".repair_logs/openat_ocreat_source_report_v136.txt"

def excluded(path: Path) -> bool:
    for part in path.parts:
        if part in EXCLUDE_DIRS:
            return True
        if any(part.startswith(prefix) for prefix in EXCLUDE_PREFIXES):
            return True
    return False

def read(path: Path) -> str:
    return path.read_text(errors="ignore")

def line_no(text: str, pos: int) -> int:
    return text.count("\n", 0, pos) + 1

def context_lines(text: str, lineno: int, radius: int = 22) -> str:
    lines = text.splitlines()
    start = max(1, lineno - radius)
    end = min(len(lines), lineno + radius)
    out = []
    for n in range(start, end + 1):
        out.append(f"{n:5d}: {lines[n-1]}")
    return "\n".join(out)

def find_balanced_function(text: str, start: int) -> tuple[int, int] | None:
    brace = text.find("{", start)
    if brace < 0:
        return None
    depth = 0
    i = brace
    while i < len(text):
        ch = text[i]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return start, i + 1
        i += 1
    return None

rs_files = [p for p in ROOT.glob("src/**/*.rs") if not excluded(p)]
if not rs_files:
    print("[ERROR] no Rust files under src/")
    sys.exit(1)

file_records = {}
all_text = ""
for p in rs_files:
    s = read(p)
    file_records[str(p)] = s
    all_text += "\n" + s

syscall_mod = ROOT / "src/syscall/mod.rs"
if not syscall_mod.exists():
    print("[ERROR] src/syscall/mod.rs not found")
    sys.exit(1)
mod_text = read(syscall_mod)

# Duplicate SYS_* arms inside individual match blocks.
duplicate_blocks = []
for idx, m in enumerate(re.finditer(r"match\s+[^{}]+\{", mod_text), 1):
    start = m.end()
    depth = 1
    i = start
    while i < len(mod_text) and depth:
        if mod_text[i] == "{":
            depth += 1
        elif mod_text[i] == "}":
            depth -= 1
        i += 1
    block = mod_text[start:i-1]
    arms = re.findall(r"^\s*(SYS_[A-Z0-9_]+)\s*=>", block, flags=re.M)
    dup = sorted({a for a in arms if arms.count(a) > 1})
    if dup:
        duplicate_blocks.append({"block": idx, "duplicates": dup})

bad_self = sorted(set(re.findall(r"^\s*(SYS_[A-Z0-9_]+)\s*=>\s*\1\b", mod_text, flags=re.M)))

function_candidates = []
for p, s in file_records.items():
    for m in re.finditer(r"(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z0-9_]*openat[A-Za-z0-9_]*)\s*\(", s):
        rng = find_balanced_function(s, m.start())
        body = s[rng[0]:rng[1]] if rng else s[m.start():m.start()+2500]
        ln = line_no(s, m.start())
        function_candidates.append({
            "file": p,
            "line": ln,
            "name": m.group(1),
            "contains_ocreat": any(x in body for x in ["O_CREAT", "O_CREATE", "CREAT", "0o100", "64"]),
            "contains_enoent": "ENOENT" in body or "NoEntry" in body or "NotFound" in body,
            "contains_flags": "flags" in body or "flag" in body,
            "contains_create_call": bool(re.search(r"\b(create|creat|mknod|new_file|insert|add_file)\b", body, re.I)),
            "sha256": hashlib.sha256(body.encode()).hexdigest(),
            "snippet": context_lines(s, ln, 35),
        })

open_related = []
for p, s in file_records.items():
    score = 0
    needles = ["SYS_OPENAT", "openat", "O_CREAT", "O_TRUNC", "ENOENT", "create", "path", "AT_FDCWD"]
    found = []
    for n in needles:
        if n in s:
            score += 1
            found.append(n)
    if score >= 3:
        # collect first few relevant contexts
        contexts = []
        for pat in ["SYS_OPENAT", "fn sys_openat", "fn openat", "O_CREAT", "ENOENT", "create"]:
            pos = s.find(pat)
            if pos >= 0:
                contexts.append({"pattern": pat, "line": line_no(s, pos), "context": context_lines(s, line_no(s, pos), 12)})
        open_related.append({"file": p, "score": score, "found": found, "contexts": contexts[:5]})

create_candidates = []
for p, s in file_records.items():
    for m in re.finditer(r"(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z0-9_]*(?:create|creat|mknod|new_file|insert_file|add_file)[A-Za-z0-9_]*)\s*\(", s, flags=re.I):
        ln = line_no(s, m.start())
        create_candidates.append({
            "file": p,
            "line": ln,
            "name": m.group(1),
            "snippet": context_lines(s, ln, 18),
        })

errno_candidates = []
for p, s in file_records.items():
    if "ENOENT" in s or "NotFound" in s or "NoEntry" in s:
        for pat in ["ENOENT", "NotFound", "NoEntry"]:
            for m in re.finditer(re.escape(pat), s):
                ln = line_no(s, m.start())
                errno_candidates.append({"file": p, "line": ln, "pattern": pat, "context": context_lines(s, ln, 8)})
                if len(errno_candidates) >= 80:
                    break
            if len(errno_candidates) >= 80:
                break
    if len(errno_candidates) >= 80:
        break

constant_candidates = []
for p, s in file_records.items():
    if any(x in s for x in ["O_CREAT", "O_TRUNC", "AT_FDCWD", "SYS_OPENAT"]):
        contexts = []
        for pat in ["SYS_OPENAT", "AT_FDCWD", "O_CREAT", "O_TRUNC", "O_RDWR", "O_RDONLY"]:
            pos = s.find(pat)
            if pos >= 0:
                contexts.append({"pattern": pat, "line": line_no(s, pos), "context": context_lines(s, line_no(s, pos), 8)})
        if contexts:
            constant_candidates.append({"file": p, "contexts": contexts})

recommendations = []
if not function_candidates:
    recommendations.append("No fn *openat* found. Check dispatcher mapping for SYS_OPENAT and whether openat is implemented under a different function name.")
else:
    for fn in function_candidates[:5]:
        if fn["contains_enoent"] and not fn["contains_ocreat"]:
            recommendations.append(f"{fn['file']}:{fn['line']} {fn['name']} has ENOENT but no explicit O_CREAT handling; likely patch target.")
        elif fn["contains_enoent"] and fn["contains_ocreat"] and not fn["contains_create_call"]:
            recommendations.append(f"{fn['file']}:{fn['line']} {fn['name']} sees O_CREAT but no obvious create call; inspect missing-path branch.")
        elif not fn["contains_create_call"]:
            recommendations.append(f"{fn['file']}:{fn['line']} {fn['name']} has no obvious create call; may delegate to VFS open that ignores O_CREAT.")

if create_candidates:
    recommendations.append("Potential VFS create helpers exist. Next patch should wire openat missing-path + O_CREAT into one of these helpers.")
else:
    recommendations.append("No obvious create helper found; next patch may need to add a regular-file create path in the VFS/in-memory FS layer.")

report = {
    "version": "v136",
    "purpose": "Locate exact source patch target for openat + O_CREAT returning ENOENT.",
    "baseline_from_v135b": "All openat test combinations returned RET_ENOENT, including O_CREAT/O_TRUNC.",
    "rust_source_count": len(rs_files),
    "duplicate_syscall_match_blocks": duplicate_blocks,
    "suspicious_self_binding_arms": bad_self,
    "function_candidates": function_candidates,
    "open_related_files": sorted(open_related, key=lambda x: (-x["score"], x["file"]))[:20],
    "create_candidates": create_candidates[:50],
    "errno_candidates": errno_candidates[:80],
    "constant_candidates": constant_candidates[:20],
    "recommendations": recommendations,
    "source_sha256": hashlib.sha256(all_text.encode()).hexdigest(),
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}

REPORT_JSON.parent.mkdir(parents=True, exist_ok=True)
REPORT_JSON.write_text(json.dumps(report, indent=2, sort_keys=True))

with REPORT_TXT.open("w") as f:
    f.write("openat O_CREAT source locator v136\n")
    f.write("=" * 80 + "\n\n")
    f.write("Baseline: v135b showed RET_ENOENT for every openat combination, including O_CREAT.\n\n")
    f.write("Recommendations:\n")
    for r in recommendations:
        f.write(f"- {r}\n")
    f.write("\nFunction candidates:\n")
    for fn in function_candidates[:10]:
        f.write("-" * 80 + "\n")
        f.write(f"{fn['file']}:{fn['line']} {fn['name']}\n")
        f.write(f"contains_ocreat={fn['contains_ocreat']} contains_enoent={fn['contains_enoent']} contains_create_call={fn['contains_create_call']} contains_flags={fn['contains_flags']}\n")
        f.write(fn["snippet"] + "\n")
    f.write("\nCreate candidates:\n")
    for c in create_candidates[:20]:
        f.write("-" * 80 + "\n")
        f.write(f"{c['file']}:{c['line']} {c['name']}\n")
        f.write(c["snippet"] + "\n")
    f.write("\nENOENT/NotFound contexts:\n")
    for e in errno_candidates[:30]:
        f.write("-" * 80 + "\n")
        f.write(f"{e['file']}:{e['line']} pattern={e['pattern']}\n")
        f.write(e["context"] + "\n")

print("[INFO] openat O_CREAT source locator v136 started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")
if duplicate_blocks:
    print("[ERROR] duplicate SYS_* arms within individual match blocks:")
    print(json.dumps(duplicate_blocks, indent=2))
    sys.exit(1)
print("[PASS] no duplicate SYS_* arms within individual match blocks")
if bad_self:
    print("[ERROR] suspicious SYS_* self-binding arms: " + ", ".join(bad_self))
    sys.exit(1)
print("[PASS] no suspicious SYS_* self-binding arms")
print(f"[INFO] function candidates: {len(function_candidates)}")
for fn in function_candidates[:5]:
    print(f"[INFO]   {fn['file']}:{fn['line']} {fn['name']} O_CREAT={fn['contains_ocreat']} ENOENT={fn['contains_enoent']} create_call={fn['contains_create_call']}")
print(f"[INFO] create candidates: {len(create_candidates)}")
for c in create_candidates[:8]:
    print(f"[INFO]   {c['file']}:{c['line']} {c['name']}")
print("[INFO] recommendations:")
for r in recommendations:
    print(f"[INFO]   {r}")
print(f"[INFO] report json: {REPORT_JSON}")
print(f"[INFO] report text: {REPORT_TXT}")
print("[PASS] openat O_CREAT source locator v136 completed")
