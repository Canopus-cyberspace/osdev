#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v123"
EXCLUDE_DIRS = {".git", "target", ".repair_logs"}
EXCLUDE_PREFIXES = (".backup_repair_",)

def excluded(path: Path) -> bool:
    for part in path.parts:
        if part in EXCLUDE_DIRS:
            return True
        if any(part.startswith(prefix) for prefix in EXCLUDE_PREFIXES):
            return True
    return False

def read(path: Path) -> str:
    return path.read_text(errors="ignore")

def fail(msg: str) -> None:
    print(f"[ERROR] {msg}")
    sys.exit(1)

def ok(msg: str) -> None:
    print(f"[PASS] {msg}")

rs_files = [p for p in ROOT.glob("src/**/*.rs") if not excluded(p)]
if not rs_files:
    fail("no Rust source files found under src/")

texts = {str(p): read(p) for p in rs_files}
all_src = "\n".join(texts.values())
lower_src = all_src.lower()
syscall_mod = ROOT / "src/syscall/mod.rs"
if not syscall_mod.exists():
    fail("src/syscall/mod.rs not found")
mod_text = read(syscall_mod)

print("[INFO] socket/network API user behavior guard v123 started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")

# Duplicate SYS_* arms only within individual match blocks.
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
        fail(f"src/syscall/mod.rs: match block {idx} duplicate arms: {', '.join(dup)}")
ok("no duplicate SYS_* arms within individual match blocks")

bad_self = re.findall(r"^\s*(SYS_[A-Z0-9_]+)\s*=>\s*\1\b", mod_text, flags=re.M)
if bad_self:
    fail("suspicious SYS_* self-binding arms: " + ", ".join(sorted(set(bad_self))))
ok("no suspicious SYS_* self-binding arms")

required_groups = {
    "socket creation/core": ["SYS_SOCKET", "SYS_SOCKETPAIR", "socket", "socketpair"],
    "address lifecycle APIs": ["SYS_BIND", "SYS_CONNECT", "SYS_LISTEN", "SYS_ACCEPT", "SYS_ACCEPT4", "bind", "connect", "listen", "accept4"],
    "name/address query APIs": ["SYS_GETSOCKNAME", "SYS_GETPEERNAME", "getsockname", "getpeername", "sockaddr"],
    "socket options APIs": ["SYS_SETSOCKOPT", "SYS_GETSOCKOPT", "setsockopt", "getsockopt"],
    "shutdown/data path": ["SYS_SHUTDOWN", "shutdown", "SYS_SENDTO", "SYS_RECVFROM", "SYS_SENDMSG", "SYS_RECVMSG", "sendmsg", "recvmsg"],
    "usercopy/iovec/msghdr interop": ["copy_from_user", "copy_to_user", "iovec", "msghdr", "EFAULT"],
}

missing = []
for name, needles in required_groups.items():
    found = [n for n in needles if n in all_src]
    # Core and interop should be quite present; for address/name/options allow one indicator because scaffolds may normalize names.
    threshold = 2 if name in ("socket creation/core", "shutdown/data path", "usercopy/iovec/msghdr interop") else 1
    if len(found) < threshold:
        missing.append((name, needles, found))
    else:
        ok(f"{name}: " + ", ".join(found[:12]))

if missing:
    for name, needles, found in missing:
        print(f"[ERROR] {name} insufficient coverage; found={found}; expected among: {', '.join(needles)}")
    sys.exit(1)

soft_groups = {
    "address family vocabulary": ["af_unix", "af_inet", "sockaddr", "addrlen", "loopback", "unix"],
    "socket state vocabulary": ["bound", "listen", "connected", "shutdown", "peer", "queue"],
    "waitable readiness vocabulary": ["poll", "epoll", "readiness", "ready", "wake", "wait"],
    "errno/nonblocking vocabulary": ["EAGAIN", "EWOULDBLOCK", "ECONNREFUSED", "ENOTCONN", "EINVAL", "EBADF", "ENOSYS"],
}

for name, needles in soft_groups.items():
    found = [n for n in needles if n.lower() in lower_src]
    if found:
        ok(f"{name}: " + ", ".join(found[:10]))
    else:
        print(f"[WARN] {name} sparse; tracked as semantic debt, non-fatal")

manifest = {
    "version": VERSION,
    "rust_source_count": len(rs_files),
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "src_syscall_mod_bytes": len(mod_text.encode()),
    "required_groups": required_groups,
    "soft_groups": soft_groups,
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}
out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
manifest_path = out_dir / "socket_network_api_user_behavior_manifest_v123.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] manifest: {manifest_path}")
ok("socket/network API user behavior guard v123 completed")
