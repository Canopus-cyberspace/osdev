#!/usr/bin/env python3
from __future__ import annotations
import json
import re
import sys
from pathlib import Path

PREFIX = "[ucompat-v129]"
PAT = re.compile(rf"{re.escape(PREFIX)}\s+([a-z0-9_]+)\s+(PASS|FAIL|SKIP)(?:\s+(.*))?$")

def parse_text(text: str):
    rows = []
    for lineno, line in enumerate(text.splitlines(), 1):
        m = PAT.search(line.strip())
        if m:
            rows.append({
                "line": lineno,
                "scenario": m.group(1),
                "result": m.group(2),
                "extra": m.group(3) or "",
                "raw": line.strip(),
            })
    return rows

def summarize(rows):
    by_result = {"PASS": 0, "FAIL": 0, "SKIP": 0}
    by_scenario = {}
    for row in rows:
        by_result[row["result"]] = by_result.get(row["result"], 0) + 1
        by_scenario.setdefault(row["scenario"], {"PASS": 0, "FAIL": 0, "SKIP": 0})
        by_scenario[row["scenario"]][row["result"]] = by_scenario[row["scenario"]].get(row["result"], 0) + 1
    return {"count": len(rows), "by_result": by_result, "by_scenario": by_scenario, "rows": rows}

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("usage: parse_external_init_conformance_evidence_v130.py <log> [<log>...]", file=sys.stderr)
        sys.exit(2)
    all_rows = []
    for arg in sys.argv[1:]:
        path = Path(arg)
        rows = parse_text(path.read_text(errors="ignore"))
        for row in rows:
            row["file"] = str(path)
        all_rows.extend(rows)
    print(json.dumps(summarize(all_rows), indent=2, sort_keys=True))
    sys.exit(0)
