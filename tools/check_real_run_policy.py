#!/usr/bin/env python3
"""
Real-run policy gate.

Run from the repository root:

    python3 tools/check_real_run_policy.py

This gate scans source files for official judge success markers outside
approved emitter files. It is intentionally conservative: when it finds a
new hardcoded parser-shaped success line, it fails and asks the developer
to route the output through a RealRunResult-based emitter.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path
from typing import Iterable


DEFAULT_CONFIG = "config/real_run_policy.json"

DEFAULT_SKIP_DIRS = {
    ".git",
    "target",
    ".repair_logs",
    "logs",
    "build",
    "dist",
    "__pycache__",
}

DEFAULT_SCAN_EXTS = {
    ".rs",
    ".c",
    ".h",
    ".S",
    ".s",
    ".sh",
    ".py",
    ".toml",
    ".mk",
    ".Makefile",
    "",
}

DEFAULT_SCAN_FILES = {
    "Makefile",
    "apply_fix.sh",
    "build.rs",
}


def load_policy(path: Path) -> dict:
    if not path.exists():
        raise SystemExit(f"policy config not found: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def is_allowed(path: Path, repo: Path, allowed_paths: set[str]) -> bool:
    rel = path.relative_to(repo).as_posix()
    return rel in allowed_paths


def should_scan(path: Path) -> bool:
    if path.name in DEFAULT_SCAN_FILES:
        return True
    if path.suffix in DEFAULT_SCAN_EXTS:
        return True
    return False


def iter_files(repo: Path, roots: Iterable[str] | None = None) -> Iterable[Path]:
    if roots:
        candidates = [repo / r for r in roots]
    else:
        candidates = [repo / "src", repo / "tools", repo / "user", repo / "kernel", repo / "Makefile", repo / "apply_fix.sh", repo / "build.rs"]

    for candidate in candidates:
        if not candidate.exists():
            continue
        if candidate.is_file():
            if should_scan(candidate):
                yield candidate
            continue
        for root, dirs, files in os.walk(candidate):
            dirs[:] = [d for d in dirs if d not in DEFAULT_SKIP_DIRS]
            root_path = Path(root)
            for name in files:
                p = root_path / name
                if should_scan(p):
                    yield p


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", default=".", help="repository root")
    parser.add_argument("--config", default=DEFAULT_CONFIG, help="policy JSON path")
    parser.add_argument("--roots", nargs="*", default=None, help="optional scan roots relative to repo")
    parser.add_argument("--allow-current-content-backed", action="store_true",
                        help="do not fail when existing allowed emitter files contain legacy content-backed output")
    args = parser.parse_args()

    repo = Path(args.repo).resolve()
    policy_path = (repo / args.config).resolve()
    policy = load_policy(policy_path)

    emitter_cfg = policy.get("official_success_emission", {})
    allowed_paths = set(emitter_cfg.get("allowed_emitter_paths", []))

    patterns = []
    for item in policy.get("forbidden_success_patterns_outside_allowlist", []):
        patterns.append((re.compile(item["regex"]), item.get("reason", "forbidden success marker")))

    violations: list[tuple[str, int, str, str]] = []

    for path in iter_files(repo, args.roots):
        try:
            text = path.read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue

        rel = path.relative_to(repo).as_posix()
        if is_allowed(path, repo, allowed_paths):
            continue

        for rx, reason in patterns:
            for m in rx.finditer(text):
                line_no = text.count("\n", 0, m.start()) + 1
                line = text.splitlines()[line_no - 1].strip()
                violations.append((rel, line_no, line, reason))

    if violations:
        print("REAL-RUN POLICY GATE: FAIL")
        print()
        print("Found parser-shaped success markers outside allowed RealRunResult emitters.")
        print("Route these outputs through emit_official_success_if_real(...) or remove them.")
        print()
        for rel, line_no, line, reason in violations:
            print(f"{rel}:{line_no}: {line}")
            print(f"  reason: {reason}")
        print()
        print("Core rule:")
        print(policy.get("one_sentence_rule", "success output must be backed by real execution"))
        return 2

    print("REAL-RUN POLICY GATE: PASS")
    print(policy.get("one_sentence_rule", "success output must be backed by real execution"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
