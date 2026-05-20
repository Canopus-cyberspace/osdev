#!/usr/bin/env python3
from __future__ import annotations
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
OUT_TXT = Path(sys.argv[1])
OUT_JSON = Path(sys.argv[2])

TARGET = ROOT / "src/mm/sv39_init_exec.rs"
FS_TARGET = ROOT / "src/syscall/fs.rs"
EXTRA_GLOBS = ["src/**/*.rs"]

def read(p: Path) -> str:
    return p.read_text(errors="ignore")

def line_no(text: str, pos: int) -> int:
    return text.count("\n", 0, pos) + 1

def numbered(text: str, start_line: int = 1) -> str:
    return "\n".join(f"{start_line + i:5d}: {line}" for i, line in enumerate(text.splitlines()))

def context_by_line(text: str, line: int, radius: int) -> str:
    lines = text.splitlines()
    start = max(1, line - radius)
    end = min(len(lines), line + radius)
    return "\n".join(f"{n:5d}: {lines[n-1]}" for n in range(start, end + 1))

def find_fn(text: str, name: str):
    m = re.search(rf"(?:pub\s+)?(?:async\s+)?fn\s+{re.escape(name)}\s*\(", text)
    if not m:
        return None
    brace = text.find("{", m.start())
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
                return {
                    "name": name,
                    "start_pos": m.start(),
                    "end_pos": i + 1,
                    "start_line": line_no(text, m.start()),
                    "end_line": line_no(text, i),
                    "body": text[m.start():i+1],
                }
        i += 1
    return None

def find_openat_functions(path: Path):
    if not path.exists():
        return []
    s = read(path)
    out = []
    for m in re.finditer(r"(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z0-9_]*openat[A-Za-z0-9_]*)\s*\(", s):
        fn = find_fn(s, m.group(1))
        if fn:
            fn["file"] = str(path)
            out.append(fn)
    return out

def grep_context(patterns, limit=80):
    rows = []
    for p in ROOT.glob("src/**/*.rs"):
        if any(part in {".git", "target", ".repair_logs"} or part.startswith(".backup_repair_") for part in p.parts):
            continue
        s = read(p)
        for pat in patterns:
            for m in re.finditer(pat, s, flags=re.I):
                ln = line_no(s, m.start())
                rows.append({
                    "file": str(p),
                    "line": ln,
                    "pattern": pat,
                    "context": context_by_line(s, ln, 10),
                })
                if len(rows) >= limit:
                    return rows
    return rows

data = {
    "version": "v137b",
    "target": str(TARGET),
    "fs_target": str(FS_TARGET),
    "sys_openat_user": None,
    "sys_openat2_user": None,
    "sys_openat_fs": None,
    "all_openat_functions": [],
    "open_create_errno_contexts": [],
    "open_file_fd_contexts": [],
}

if TARGET.exists():
    s = read(TARGET)
    for name in ["sys_openat_user", "sys_openat2_user"]:
        fn = find_fn(s, name)
        if fn:
            fn["file"] = str(TARGET)
            data[name] = {k: v for k, v in fn.items() if k != "body"}
            data[name]["body"] = fn["body"]
    data["all_openat_functions"].extend([{k: v for k, v in fn.items() if k != "body"} | {"body": fn["body"]} for fn in find_openat_functions(TARGET)])

if FS_TARGET.exists():
    s = read(FS_TARGET)
    fn = find_fn(s, "sys_openat")
    if fn:
        fn["file"] = str(FS_TARGET)
        data["sys_openat_fs"] = {k: v for k, v in fn.items() if k != "body"}
        data["sys_openat_fs"]["body"] = fn["body"]
    data["all_openat_functions"].extend([{k: v for k, v in fn.items() if k != "body"} | {"body": fn["body"]} for fn in find_openat_functions(FS_TARGET)])

data["open_create_errno_contexts"] = grep_context([
    r"O_CREAT", r"O_TRUNC", r"ENOENT", r"NotFound", r"NoEntry", r"create", r"creat", r"openat"
], limit=120)
data["open_file_fd_contexts"] = grep_context([
    r"FileDescriptor", r"FdTable", r"alloc_fd", r"insert.*fd", r"open_file", r"File::", r"struct .*File", r"trait .*File"
], limit=120)

OUT_JSON.parent.mkdir(parents=True, exist_ok=True)
OUT_JSON.write_text(json.dumps(data, indent=2, sort_keys=True))

with OUT_TXT.open("w") as f:
    f.write("v137b openat O_CREAT context dump\n")
    f.write("=" * 100 + "\n\n")
    f.write("WHY THIS EXISTS\n")
    f.write("- v135b/v136 proved all openat combinations returned RET_ENOENT.\n")
    f.write("- v137 selected sys_openat_user but could not safely infer the exact ENOENT branch.\n")
    f.write("- This report contains full function bodies needed for a source-specific v137c patch.\n\n")

    for label, key in [
        ("PRIMARY TARGET: src/mm/sv39_init_exec.rs::sys_openat_user", "sys_openat_user"),
        ("RELATED: src/mm/sv39_init_exec.rs::sys_openat2_user", "sys_openat2_user"),
        ("RELATED: src/syscall/fs.rs::sys_openat", "sys_openat_fs"),
    ]:
        f.write("\n" + "=" * 100 + "\n")
        f.write(label + "\n")
        f.write("=" * 100 + "\n")
        item = data.get(key)
        if item:
            f.write(f"FILE: {item['file']}\n")
            f.write(f"LINES: {item['start_line']}-{item['end_line']}\n\n")
            f.write(numbered(item["body"], item["start_line"]))
            f.write("\n")
        else:
            f.write("<not found>\n")

    f.write("\n" + "=" * 100 + "\n")
    f.write("OPEN/CREATE/ENOENT CONTEXTS\n")
    f.write("=" * 100 + "\n")
    for row in data["open_create_errno_contexts"][:80]:
        f.write("\n" + "-" * 100 + "\n")
        f.write(f"{row['file']}:{row['line']} pattern={row['pattern']}\n")
        f.write(row["context"] + "\n")

    f.write("\n" + "=" * 100 + "\n")
    f.write("FD/FILE TABLE CONTEXTS\n")
    f.write("=" * 100 + "\n")
    for row in data["open_file_fd_contexts"][:80]:
        f.write("\n" + "-" * 100 + "\n")
        f.write(f"{row['file']}:{row['line']} pattern={row['pattern']}\n")
        f.write(row["context"] + "\n")

print("[INFO] v137b context dump generated")
print(f"[INFO] context text: {OUT_TXT}")
print(f"[INFO] context json: {OUT_JSON}")
primary = data.get("sys_openat_user")
if primary:
    print(f"[PASS] sys_openat_user context found: {primary['file']}:{primary['start_line']}-{primary['end_line']}")
else:
    print("[ERROR] sys_openat_user context not found")
    sys.exit(1)
print("[PASS] openat O_CREAT context dump v137b completed")
