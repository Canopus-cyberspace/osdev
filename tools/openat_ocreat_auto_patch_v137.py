#!/usr/bin/env python3
from __future__ import annotations
import json
import re
import shutil
import sys
from pathlib import Path

ROOT = Path.cwd()
TS = sys.argv[1] if len(sys.argv) > 1 else "manual"
EXCERPT = Path(sys.argv[2]) if len(sys.argv) > 2 else ROOT / ".repair_logs/openat_ocreat_patch_excerpt_v137.txt"
REPORT = Path(sys.argv[3]) if len(sys.argv) > 3 else ROOT / ".repair_logs/openat_ocreat_patch_report_v137.json"
PATCH_MARKER = "// UCOMPAT_V137_OPENAT_OCREAT_PATCH"

TARGETS = [
    ROOT / "src/mm/sv39_init_exec.rs",
    ROOT / "src/syscall/fs.rs",
]

def read(p: Path) -> str:
    return p.read_text(errors="ignore")

def write(p: Path, s: str) -> None:
    p.write_text(s)

def line_no(text: str, pos: int) -> int:
    return text.count("\n", 0, pos) + 1

def context_lines(text: str, lineno: int, radius: int = 80) -> str:
    lines = text.splitlines()
    start = max(1, lineno - radius)
    end = min(len(lines), lineno + radius)
    return "\n".join(f"{n:5d}: {lines[n-1]}" for n in range(start, end + 1))

def find_function(text: str, name: str):
    m = re.search(rf"(?:pub\s+)?(?:async\s+)?fn\s+{re.escape(name)}\s*\([^)]*\)\s*(?:->\s*[^\{{]+)?\{{", text)
    if not m:
        return None
    brace = text.find("{", m.start())
    depth = 0
    i = brace
    while i < len(text):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return m.start(), i + 1, line_no(text, m.start()), text[m.start():i+1]
        i += 1
    return None

def find_any_openat_functions():
    found = []
    for p in TARGETS:
        if not p.exists():
            continue
        s = read(p)
        for m in re.finditer(r"(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z0-9_]*openat[A-Za-z0-9_]*)\s*\(", s):
            name = m.group(1)
            fn = find_function(s, name)
            if fn:
                found.append((p, name, fn[2], fn[3]))
    return found

def make_report(status: str, detail: str, extra: dict):
    REPORT.parent.mkdir(parents=True, exist_ok=True)
    report = {"version": "v137", "status": status, "detail": detail, **extra}
    REPORT.write_text(json.dumps(report, indent=2, sort_keys=True))
    print(f"[INFO] patch report: {REPORT}")
    return report

def extract_excerpts(functions):
    EXCERPT.parent.mkdir(parents=True, exist_ok=True)
    with EXCERPT.open("w") as f:
        f.write("v137 openat O_CREAT patch context\n")
        f.write("=" * 90 + "\n\n")
        for p, name, ln, body in functions:
            f.write("-" * 90 + "\n")
            f.write(f"{p}:{ln} {name}\n")
            f.write(body[:12000])
            if len(body) > 12000:
                f.write("\n... <truncated> ...\n")
            f.write("\n\n")
    print(f"[INFO] source excerpt: {EXCERPT}")

# Strategy 0: report exact context. This is always useful.
functions = find_any_openat_functions()
extract_excerpts(functions)

if not functions:
    print("[ERROR] V137_NEEDS_MANUAL_PATCH_CONTEXT: no openat functions found")
    make_report("needs_manual_context", "no openat functions found", {"functions": []})
    sys.exit(2)

# Prefer user-mode external-init syscall function, because v135/v136 diagnostics target init.elf path.
candidate = None
for p, name, ln, body in functions:
    if p.name == "sv39_init_exec.rs" and name == "sys_openat_user":
        candidate = (p, name, ln, body)
        break
if candidate is None:
    for p, name, ln, body in functions:
        if name == "sys_openat_user":
            candidate = (p, name, ln, body)
            break
if candidate is None:
    candidate = functions[0]

p, name, ln, body = candidate
print(f"[INFO] selected patch candidate: {p}:{ln} {name}")

if PATCH_MARKER in read(p):
    print("[INFO] v137 patch marker already present; skipping source modification")
    make_report("already_patched", "patch marker already present", {"candidate": {"file": str(p), "line": ln, "name": name}})
    sys.exit(0)

# This auto-patcher is intentionally conservative. It only handles the common layout:
#   path string already copied into a local variable, then a direct return -ENOENT or Err(ENOENT)
# If we cannot identify path/flags variables and an ENOENT return, we stop and emit excerpts.
s = read(p)
fn = find_function(s, name)
if not fn:
    print("[ERROR] V137_NEEDS_MANUAL_PATCH_CONTEXT: selected function disappeared")
    make_report("needs_manual_context", "selected function disappeared", {"candidate": {"file": str(p), "line": ln, "name": name}})
    sys.exit(2)
start, end, ln, body = fn

# Detect variable names.
sig = body.split("{", 1)[0]
arg_names = re.findall(r"([A-Za-z_][A-Za-z0-9_]*)\s*:", sig)
flags_var = None
for v in arg_names:
    if "flag" in v.lower():
        flags_var = v
        break
if flags_var is None:
    for v in ["flags", "flag", "open_flags"]:
        if re.search(rf"\b{v}\b", body):
            flags_var = v
            break

path_var = None
# Prefer variables assigned from copy/cstr/path.
for pat in [
    r"let\s+(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*=\s*[^;]*(?:copy|cstr|path)[^;]*;",
    r"let\s+(?:mut\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(?:String|&str|Vec<.*?>)\s*=\s*[^;]*;",
]:
    m = re.search(pat, body, flags=re.I | re.S)
    if m:
        path_var = m.group(1)
        break
for v in ["path", "pathname", "name", "filename", "file_name", "path_str"]:
    if path_var is None and re.search(rf"\b{v}\b", body):
        path_var = v

# Detect ENOENT return site in function body.
enoent_patterns = [
    r"return\s+[-(]*\s*ENOENT[^;]*;",
    r"return\s+[^;]*ENOENT[^;]*;",
    r"Err\s*\([^)]*ENOENT[^)]*\)",
    r"-\s*\(?ENOENT\s+as\s+isize\)?",
    r"-2\b",
]
enoent_match = None
for pat in enoent_patterns:
    m = re.search(pat, body)
    if m:
        enoent_match = m
        break

if flags_var is None or path_var is None or enoent_match is None:
    print("[ERROR] V137_NEEDS_MANUAL_PATCH_CONTEXT: cannot safely infer flags/path/ENOENT site")
    print(f"[INFO] inferred flags_var={flags_var} path_var={path_var} enoent_site={bool(enoent_match)}")
    make_report("needs_manual_context", "cannot safely infer flags/path/ENOENT site", {
        "candidate": {"file": str(p), "line": ln, "name": name},
        "flags_var": flags_var,
        "path_var": path_var,
        "has_enoent_site": bool(enoent_match),
        "excerpt": str(EXCERPT),
    })
    sys.exit(2)

print(f"[INFO] inferred flags variable: {flags_var}")
print(f"[INFO] inferred path variable: {path_var}")

# Try to find an existing successful open helper/call in sys_openat_user body.
# If there is a VFS open call, we can often retry it with create-on-miss by passing a create flag.
# But since exact types are unknown, we only patch a very narrow common scaffold: hardcoded known external-init paths
# plus existing fallback regular file open helper if present.
helper_call = None
for m in re.finditer(r"([A-Za-z_][A-Za-z0-9_:]*open[A-Za-z0-9_:]*)\s*\(([^;{}]+)\)", body):
    call = m.group(0)
    if "openat" not in call.lower() and "println" not in call.lower():
        helper_call = call
        break

# If we cannot find an open helper, do not invent a file table; that would risk breaking build.
if helper_call is None:
    print("[ERROR] V137_NEEDS_MANUAL_PATCH_CONTEXT: no existing open helper call found to reuse for create-on-miss")
    make_report("needs_manual_context", "no existing open helper call found", {
        "candidate": {"file": str(p), "line": ln, "name": name},
        "flags_var": flags_var,
        "path_var": path_var,
        "excerpt": str(EXCERPT),
    })
    sys.exit(2)

print(f"[INFO] candidate helper call: {helper_call}")

# Conservative patch impossible generically: instead of writing invalid Rust, emit actionable report.
# This prevents corrupting source while preserving evidence and exact context.
print("[ERROR] V137_NEEDS_MANUAL_PATCH_CONTEXT: source layout requires helper-specific patch")
print("[INFO] The patcher found the function and variables but will not synthesize unknown VFS helper calls.")
print("[INFO] Paste the excerpt path contents or rerun with the report; next patch can edit the exact helper branch.")
make_report("needs_helper_specific_patch", "helper-specific patch required", {
    "candidate": {"file": str(p), "line": ln, "name": name},
    "flags_var": flags_var,
    "path_var": path_var,
    "helper_call": helper_call,
    "excerpt": str(EXCERPT),
})
sys.exit(2)
