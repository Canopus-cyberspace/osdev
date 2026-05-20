#!/usr/bin/env python3
from __future__ import annotations
import hashlib
import importlib.util
import json
import re
import sys
from pathlib import Path

ROOT = Path.cwd()
VERSION = "v131"
EXCLUDE_DIRS = {".git", "target", ".repair_logs"}
EXCLUDE_PREFIXES = (".backup_repair_",)
DEFAULT_PREFIX = "[ucompat-v129]"
DEFAULT_RESULTS = {"PASS", "FAIL", "SKIP"}

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

def load_protocol() -> tuple[dict, str]:
    candidates = [
        ROOT / "tools/external_init_conformance_marker_protocol_v129.json",
        ROOT / "tools/external_init_conformance_runner_plan_v128.json",
    ]
    for p in candidates:
        if not p.exists():
            continue
        try:
            doc = json.loads(p.read_text())
            if "scenario_marker_prefix" in doc:
                return doc, str(p)
            contract = doc.get("contract") or doc.get("runner_contract") or {}
            if "scenario_marker_prefix" in contract:
                out = {
                    "scenario_marker_prefix": contract["scenario_marker_prefix"],
                    "result_markers": contract.get("result_markers", ["PASS", "FAIL", "SKIP"]),
                    "scenario_results": doc.get("scenario_results", {}),
                }
                return out, str(p)
        except Exception as exc:
            print(f"[WARN] failed to parse protocol candidate {p}: {exc}")
    return {"scenario_marker_prefix": DEFAULT_PREFIX, "result_markers": sorted(DEFAULT_RESULTS), "scenario_results": {}}, "<default>"

def import_parser(parser_path: Path):
    spec = importlib.util.spec_from_file_location("v130_parser", parser_path)
    if spec is None or spec.loader is None:
        fail("unable to import evidence parser")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

rs_files = [p for p in ROOT.glob("src/**/*.rs") if not excluded(p)]
tool_files = [p for p in ROOT.glob("tools/*") if p.is_file() and not excluded(p)]
if not rs_files:
    fail("no Rust source files found under src/")

syscall_mod = ROOT / "src/syscall/mod.rs"
if not syscall_mod.exists():
    fail("src/syscall/mod.rs not found")
mod_text = read(syscall_mod)

protocol, protocol_source = load_protocol()
prefix = protocol.get("scenario_marker_prefix") or DEFAULT_PREFIX
result_markers = set(protocol.get("result_markers") or sorted(DEFAULT_RESULTS))

print("[INFO] external-init conformance evidence aggregator v131 started")
print(f"[INFO] rust source files scanned: {len(rs_files)}")
print(f"[INFO] tool files scanned: {len(tool_files)}")
print(f"[INFO] protocol source: {protocol_source}")
print(f"[INFO] protocol prefix: {prefix}")

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

if not re.match(r"^\[ucompat-v\d+[a-z]*\]$", prefix):
    fail(f"invalid scenario marker prefix: {prefix}")
if not DEFAULT_RESULTS.issubset(result_markers):
    fail(f"result marker enum incomplete: {sorted(result_markers)}")
ok("marker protocol accepted for aggregation")

parser_path = ROOT / "tools/parse_external_init_conformance_evidence_v130.py"
if not parser_path.exists():
    fail("v130 parser missing after install")
parser = import_parser(parser_path)
ok("evidence parser import succeeded")

synthetic_lines = [
    f"{prefix} vfs_open_rw_lseek PASS bytes=5",
    f"{prefix} mmap_brk_basic PASS pages=1",
    f"[kernel] {prefix} signal_delivery_return SKIP reason=not-yet-implemented",
    f"{prefix} socketpair_msg FAIL errno=ENOSYS",
]
out_dir = ROOT / ".repair_logs"
out_dir.mkdir(exist_ok=True)
synthetic_path = out_dir / "external_init_conformance_synthetic_evidence_v131.log"
synthetic_path.write_text("\n".join(synthetic_lines) + "\n")
synthetic_rows = parser.parse_text(synthetic_path.read_text(errors="ignore"))
if len(synthetic_rows) != 4:
    fail(f"synthetic evidence aggregation failed; rows={synthetic_rows}")
synthetic_summary = parser.summarize(synthetic_rows)
if synthetic_summary["by_result"].get("PASS") != 2 or synthetic_summary["by_result"].get("FAIL") != 1 or synthetic_summary["by_result"].get("SKIP") != 1:
    fail(f"synthetic evidence summary mismatch: {synthetic_summary}")
ok("synthetic evidence aggregation passed")

observed_rows = []
for p in sorted((ROOT / ".repair_logs").glob("*.log"))[-80:]:
    if p.name == synthetic_path.name:
        continue
    try:
        rows = parser.parse_text(p.read_text(errors="ignore"))
    except Exception:
        continue
    for row in rows:
        row["file"] = str(p)
    observed_rows.extend(rows)

observed_summary = parser.summarize(observed_rows)
if observed_rows:
    ok(f"observed existing conformance evidence rows: {len(observed_rows)}")
else:
    print("[INFO] no observed per-scenario evidence rows yet; expected before init.elf multi-case emission")

init_builder = ROOT / "user/build_init_elf.py"
if not init_builder.exists():
    fail("user/build_init_elf.py missing")
init_text = init_builder.read_text(errors="ignore")
needed = ["hello from external init.elf", "syscall write", "init.elf"]
found_needed = [x for x in needed if x in init_text]
if len(found_needed) < 2:
    fail("external-init builder marker readiness insufficient")
ok("external-init single-marker smoke remains available: " + ", ".join(found_needed))

manifest = {
    "version": VERSION,
    "phase": "evidence-aggregation",
    "protocol_source": protocol_source,
    "scenario_marker_prefix": prefix,
    "result_markers": sorted(result_markers),
    "parser_path": str(parser_path),
    "synthetic_evidence_path": str(synthetic_path),
    "synthetic_summary": synthetic_summary,
    "observed_summary": observed_summary,
    "observed_rows_sample": observed_rows[:20],
    "rust_source_count": len(rs_files),
    "tool_file_count": len(tool_files),
    "src_syscall_mod_sha256": hashlib.sha256(mod_text.encode()).hexdigest(),
    "src_syscall_mod_bytes": len(mod_text.encode()),
    "parser_sha256": hashlib.sha256(parser_path.read_bytes()).hexdigest(),
    "synthetic_sha256": hashlib.sha256(synthetic_path.read_bytes()).hexdigest(),
    "excluded_dirs": sorted(EXCLUDE_DIRS),
    "excluded_prefixes": list(EXCLUDE_PREFIXES),
}
manifest_path = out_dir / "external_init_conformance_evidence_aggregator_manifest_v131.json"
manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
print(f"[INFO] parser: {parser_path}")
print(f"[INFO] synthetic evidence: {synthetic_path}")
print(f"[INFO] manifest: {manifest_path}")
ok("external-init conformance evidence aggregator v131 completed")
