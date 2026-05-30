# Prompt Templates

Reusable task prompts for Claude Code / Codex / DeepSeek-style agents working
on the `source/` kernel. Each template is self-contained: paste it into a new
agent session with the task-specific details filled in.

Before using any template, the agent must read:
- `AGENTS.md`
- `source/RULES.md`
- `source/ARCHITECTURE.md`

---

## 1. Plan-Only Narrow Task

```
Read AGENTS.md, source/RULES.md, and source/ARCHITECTURE.md first.

TASK: [one-sentence description]

SCOPE:
- Plan only. Do not write code.
- Identify all files that would need to change.
- Identify every interface, type, and boundary affected.

FILES TO INSPECT:
- [list specific files]

FILES ALLOWED TO CHANGE: none (plan only).

HARD CONSTRAINTS:
- No path selection, fake data, logs, or permanent gates.
- source/core and source/kernel must stay architecture-neutral.
- RISC-V and LoongArch must stay at the same abstraction level.

VALIDATION:
- List the architecture-boundary grep that would verify no arch leakage.
- List the unsafe-in-core grep that would verify safety.
- List the forbidden-pattern greps that would verify no regressions.

REPORT:
- Files to touch (with rationale).
- Interfaces/types to add or modify.
- Ownership: which subsystem owns each new piece.
- Any temporary gates needed, with removal timeline.
- Whether src/, source.1/, or root build files would be affected.
```

---

## 2. Apply Narrow Correctness Fix

```
Read AGENTS.md, source/RULES.md, and source/ARCHITECTURE.md first.

TASK: [exact bug or correctness issue]

SCOPE:
- Fix only the described issue.
- Do not refactor surrounding code.
- Do not add features, gates, or abstractions beyond the fix.

FILES TO INSPECT:
- [list specific files that contain the bug]

FILES ALLOWED TO CHANGE:
- [list specific files, no wildcards]

HARD CONSTRAINTS:
- No fake success, logs, or hardcoded paths.
- No unsafe in source/core.
- No architecture imports in source/core or source/kernel.
- No permanent gates.
- Keep hot paths short: no allocation, formatting, or discovery in trap/syscall/scheduler paths.
- Line limits are review triggers, not hard limits. Do not truncate or delete code to satisfy line count.
- Decompose by owner, execution tier, or hardware boundary, not mechanically by line count.
- Avoid tiny-file explosion: a 20–50 line focused addition stays in the existing owner module.
- If adding behavior to a file over 1200 lines, justify why it belongs there.

VALIDATION:
- Run: [list specific validation commands from source/VALIDATION.md]
- Architecture boundary grep must be clean.
- Forbidden-pattern grep must be clean.

REPORT:
- Exact lines changed and why.
- Validation results.
- Whether the fix touches src/, source.1/, or root build files.
```

---

## 3. BSP Hardware Boundary Implementation

```
Read AGENTS.md, source/RULES.md, and source/ARCHITECTURE.md first.

TASK: implement [hardware primitive] for [RISC-V | LoongArch | both].

SCOPE:
- BSP code only: arch/[riscv64|loongarch64]/.
- Hardware/ISA mechanics only. No OS semantics.
- Provide the same neutral contract for both architectures.

FILES TO INSPECT:
- source/arch/[riscv64|loongarch64]/[specific module]
- source/core/[relevant trait or contract file]
- src/arch/[riscv64|loongarch64]/[legacy reference — study only]

FILES ALLOWED TO CHANGE:
- source/arch/[riscv64|loongarch64]/[specific files]

HARD CONSTRAINTS:
- No syscall semantics, VFS, scheduler policy, process lifecycle in BSP code.
- Unsafe hardware actions must have exactly one narrow unsafe entry point.
- Use BoundaryMode (Inspect/Prepare/ApplyUnsafe).
- Prepared<T> crosses the unsafe boundary; ApplyUnsafe produces a real side effect.
- No PlanOnly/ValidateOnly duplicate modes.
- No boolean dispatch for semantic modes.
- For LoongArch: hardware uncertainty → typed NotReady/Unsupported/DiscoveryRequired, not fake success.
- No debug logs in production.

VALIDATION:
- cargo check && cargo build for both targets.
- Architecture boundary grep: no arch::riscv64 in source/core or source/kernel.
- Unsafe-in-core grep must be clean.
- Forbidden-pattern grep must be clean.

REPORT:
- Files changed.
- Provider contracts added or implemented.
- Which BoundaryMode variants are used and why.
- Whether the same neutral contract is exposed on both architectures.
- Whether src/, source.1/, or root build files were touched.
```

---

## 4. Core Subsystem Implementation

```
Read AGENTS.md, source/RULES.md, and source/ARCHITECTURE.md first.

TASK: implement [subsystem name] in source/core/[subsystem]/.

SCOPE:
- Shared core only. No architecture-specific code.
- Real kernel semantics. No defensive scaffolding.

FILES TO INSPECT:
- source/core/[subsystem]/[existing files]
- source/RULES.md sections relevant to this subsystem.
- src/[legacy reference — study only, do not copy].

FILES ALLOWED TO CHANGE:
- source/core/[subsystem]/[specific files]

HARD CONSTRAINTS:
- No unsafe blocks in source/core.
- No arch::riscv64 or arch::loongarch64 imports.
- No fake success, logs, hardcoded paths, or testcase branches.
- Prefer real semantics: every field/variant must represent a real runtime fact.
- No safety-theater layers (CONFIRMED, BY_DEFAULT, called_by_default, etc.).
- One lifecycle state field per object.
- No placeholder production steps with None.
- Simple containers behind replacement-ready interfaces.
- Hot paths must be allocation-free, logging-free, policy-light.
- Document path tier for each performance-sensitive function (BootPath/SetupPath/SlowPath/HotPath/IrqPath).
- Do not create mod.rs files. Use named gateway files (declaration/re-export only, under 150 lines).
- Implementation goes in owner-specific files under matching directories.
- No types.rs, utils.rs, common.rs, or helpers.rs dumping grounds.

VALIDATION:
- cargo check && cargo build for both targets.
- Architecture boundary grep must be clean.
- Unsafe-in-core grep must be clean.
- Forbidden-pattern + hardcoded-path + log/serial + official-marker greps must be clean.

REPORT:
- Subsystem completed or advanced.
- Path-tier assignments.
- Complexity expectations for common operations.
- Interface replacement readiness.
- Whether src/, source.1/, or root build files were touched.
- Files over 800/1200 lines after the change, with justification for any file that grew past a threshold.
```

---

## 5. Refactor Without Behavior Change

```
Read AGENTS.md, source/RULES.md, and source/ARCHITECTURE.md first.

TASK: refactor [what] in [where]. No behavior change allowed.

SCOPE:
- Restructure code only. Output, syscall behavior, and QEMU behavior must not change.
- Reduce duplication, collapse redundant gates, or replace with allowed pattern.

FILES TO INSPECT:
- [list files to refactor and their callers]

FILES ALLOWED TO CHANGE:
- [specific files]

HARD CONSTRAINTS:
- No behavioral change. Same syscall results, same output, same control flow for all existing paths.
- Remove, don't add: collapse duplicate states, remove safety-theater layers, merge redundant policy.
- Replace PlanOnly/ValidateOnly with BoundaryMode if they produce the same result.
- Replace boolean dispatch with mode enum dispatch.
- Keep one lifecycle state field per object.
- No permanent gates.
- Line limits are review triggers, not hard limits. Do not truncate or delete code to satisfy line count.
- Decompose by owner, execution tier, or hardware boundary. Do not split mechanically.
- Avoid tiny-file explosion. Do not create new trait/provider/plan layers just because code moved.
- Splitting must not lengthen hot paths with extra wrappers or adapters.
- Report any file exceeding 800/1200 lines after the refactor, with justification.

VALIDATION:
- cargo check && cargo build for both targets.
- All greps from source/VALIDATION.md must be clean.
- If QEMU baseline exists, request explicit re-run; do not run QEMU by default.

REPORT:
- What was removed and what it was replaced with.
- Confirmation of no behavioral change.
- State-vocabulary audit: all state/enum/mode names mean exactly one thing.
```

---

## 6. Performance Convergence Pass

```
Read AGENTS.md, source/RULES.md, and source/ARCHITECTURE.md first.

TASK: optimize [subsystem/operation] to meet [specific complexity/behavior target].

SCOPE:
- Hot-path tightening only. Do not redesign subsystem semantics.
- Remove allocation, formatting, discovery, or plan construction from hot paths.

FILES TO INSPECT:
- [hot-path files]
- [cache-hit files if applicable]

FILES ALLOWED TO CHANGE:
- [specific files]

HARD CONSTRAINTS:
- Hot paths must be allocation-free, logging-free, discovery-free.
- Cache-hit paths must not call provider I/O.
- Context switch must consume prepared context; no rebuild.
- Syscall dispatch must be compact and allocation-free for simple syscalls.
- Linear scans replaced or bounded.
- No production logs for benchmarking.
- Performance-sensitive functions must be assigned to exact tier.

VALIDATION:
- cargo check && cargo build for both targets.
- Affected path, old complexity, new complexity documented.
- Regression check: no hot path began allocating/formatting/logging.

REPORT:
- Affected paths and their tiers.
- Old vs new complexity for each operation.
- Allocation behavior, locking behavior, cache-hit behavior.
- Confirmed: no fake success, no permanent gates, no arch leakage.
```

---

## 7. Validation-Only Pass

```
Read AGENTS.md, source/RULES.md, and source/ARCHITECTURE.md.

TASK: run the full validation suite from source/VALIDATION.md. Do not edit code.

SCOPE:
- Read-only: grep, cargo check, cargo build, make all.

HARD CONSTRAINTS:
- No code editing.
- No QEMU execution.
- No official evaluation.
- Do not run cargo unless Rust files exist.

VALIDATION:
- All greps from source/VALIDATION.md.
- cargo check + cargo build for both targets.
- make all.

REPORT:
- Every grep result (clean or hits).
- Build status for each target.
- Whether any forbidden pattern, arch leakage, or hardcoded path was found.
```

---

## 8. QEMU-Only-on-Request Pass

```
Read AGENTS.md, source/RULES.md, and source/ARCHITECTURE.md.

TASK: run QEMU smoke test for [RISC-V | LoongArch | both]. This pass is
explicitly requested by the user.

SCOPE:
- Build kernel artifacts.
- Boot in QEMU.
- Observe console output.
- Report what the kernel actually does.

HARD CONSTRAINTS:
- Do not fake output.
- Do not add debug logs for the QEMU run.
- Run only because the user explicitly requested it.
- Do not rerun official evaluation.

RUN:
- [QEMU command with exact flags]

REPORT:
- Console output verbatim.
- Whether boot reached expected stage.
- Any panic, trap, or unexpected behavior.
- Do not claim score or test pass from smoke run.
```

---

## 9. Review Patch for Fake Success / Gate / Architecture Leakage

```
Read AGENTS.md, source/RULES.md, and source/ARCHITECTURE.md first.

TASK: review the current diff for violations. Do not edit unless violations found.

SCOPE:
- Check for: fake success, security-theater layers, boolean dispatch, duplicate
  lifecycle state, permanent gates, arch leakage into core, hardcoded paths,
  debug logs, fake sector/rootfs data, testcase-specific branches, placeholder
  None steps, unused enum variants.

HARD CONSTRAINTS:
- Review only. Flag violations; do not fix unless asked.
- Report severity: P0 (blocks correctness), P1 (design debt), P2 (style).

CHECK:
- All greps from source/VALIDATION.md.
- Diff inspection for BoundaryMode misuse, duplicate state fields, boolean
  dispatch, safety-theater names, new gates.

REPORT:
- Every violation found (file, line, severity, rule that was violated).
- Whether src/, source.1/, or root build files were touched.
- Whether the diff introduces compiler warnings.
```

---

## Core Reminders — All Templates

These apply to every template above:

| Rule | Reference |
|------|-----------|
| No fake success | RULES.md §6B, §17 |
| No boot self-tests | RULES.md §5 |
| No debug logs in production hot paths | RULES.md §6C |
| No permanent gates | RULES.md §6A |
| No hardcoded paths (`/musl`, `/busybox`, `/bin`, `/lib`, `basic`) | RULES.md §16A |
| No src/ edits unless explicitly requested | AGENTS.md |
| No source.1/ edits | AGENTS.md |
| No root build file edits unless explicitly requested | AGENTS.md |
| No path selection unless explicitly requested | RULES.md §16A |
| `source/core` and `source/kernel` stay architecture-neutral | RULES.md §8 |
| RISC-V and LoongArch at same neutral abstraction level | RULES.md §1A |
| Prefer real semantics over defensive scaffolding | RULES.md §6B |
| Keep hot paths short: no alloc, no format, no discovery | RULES.md §6C |
| Run QEMU/official evaluation only when explicitly requested | AGENTS.md |
| Line limits are review triggers, not truncation rules | RULES.md §25A |
| New modules require new owner, tier, boundary, syscall group, or public API | RULES.md §25B |
| Avoid dumping-ground files (types/utils/common/helpers) | RULES.md §25C |
| Splits must not add hot-path wrappers or adapters | RULES.md §25E |
| Advance one real executable path; no broad scaffolding without a vertical slice | RULES.md §26A |
| No public traits/contracts/plans without a real caller | RULES.md §26B |
| Setup validates once; hot paths consume prepared state | RULES.md §26C |
| Do not mix unrelated high-risk categories in one patch | RULES.md §26D |
| Report scaffolding budget: any scaffold needs removal/collapse condition | RULES.md §26E |
| No mod.rs files; use named gateway files for module declarations | RULES.md §27A |
| Gateway files are declaration/re-export only; implementation in owner files | RULES.md §27C |
| No types.rs/utils.rs/common.rs/helpers.rs dumping grounds | RULES.md §25C |
