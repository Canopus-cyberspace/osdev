# Validation Command Reference

Commands for verifying `source/` correctness without running QEMU or official
evaluation. All commands are documentation-safe and read-only.

---

## Fast Documentation-Only Validation

For markdown/plan/documentation-only tasks. No Rust toolchain needed.

```bash
find source -path source/target -prune -o -type f \( -name '*.md' -o -name 'AGENTS.md' \) -print
grep -RInE "ucompat|UCOMPAT|legacy|history_bus|evidence_bus|run_v[0-9]|self_test" source/ || true
grep -RInE '"/musl|"/busybox|"/bin|"/lib|basic|busybox' source/ || true
```

---

## Standard Source Rust Validation

Run when Rust files in `source/` are created or modified. Do not run for
documentation-only changes.

```bash
cargo check --manifest-path source/Cargo.toml
cargo fmt --manifest-path source/Cargo.toml
cargo build --manifest-path source/Cargo.toml --target riscv64gc-unknown-none-elf --bin source-riscv64-kernel
cargo build --manifest-path source/Cargo.toml --target loongarch64-unknown-none --bin source-loongarch64-kernel
make all
```

No compiler warnings allowed.

---

## Architecture Boundary Grep

Verify shared core and kernel do not import architecture-specific modules.

```bash
grep -RInE "arch::riscv64|arch::loongarch64|riscv64::|loongarch64::" source/core source/kernel || true
```

Empty output expected. Any hit is an architecture-boundary violation.

---

## Unsafe-in-Core Grep

```bash
grep -RIn "unsafe" source/core || true
```

Empty output expected. `source/core` must contain no `unsafe` blocks. Unsafe
belongs in BSP architecture crates or narrow unsafe-boundary wrappers in
`source/kernel`.

---

## Forbidden Historical-Pattern Grep

```bash
grep -RInE --include='*.rs' "ucompat|UCOMPAT|legacy|history_bus|evidence_bus|run_v[0-9]|self_test" source/ || true
```

Empty output expected. See `source/RULES.md` section 3.

---

## Hardcoded-Path Grep

```bash
grep -RInE --include='*.rs' '"/musl|"/busybox|"/bin|"/lib|basic|busybox' source/core source/kernel source/arch source/bin || true
```

Empty output expected. See `source/RULES.md` section 16A.

---

## Log / Serial Grep

```bash
grep -RInE --include='*.rs' "println!|early_console_write|UART|serial" source/ || true
```

Allowed only in:
- BSP low-level console implementation
- `source/official/judge_output`
- Fatal panic path

All other hits are violations. See `source/RULES.md` section 4.

---

## Official Marker Grep

```bash
grep -RInE --include='*.rs' "testcase .*success|testcase .*fail|OS COMP TEST GROUP" source/ || true
```

Allowed only in `source/official/judge_output`. All other hits are violations.
See `source/RULES.md` section 17.

---

## Safety-Theater Grep

```bash
grep -RInE --include='*.rs' "CONFIRMED|_BY_DEFAULT|called_by_default|executes_by_default|ExplicitOnly|PreparedButNotExecuted|PlanOnly|ValidateOnly|DryRunReady|NotCalled" source/ || true
```

Forbidden names per `source/RULES.md` section 6B. Exceptions: `ValidateOnly` and
`PlanOnly` allowed only when they produce observably different results from each
other. `BoundaryMode` enum (Inspect/Prepare/ApplyUnsafe) is the replacement.

---

## Duplicate Lifecycle State Grep

```bash
grep -RInE --include='*.rs' "readiness.*UnsafeBoundaryStatus|UnsafeBoundaryStatus.*readiness|state.*result.*EnterUserReady|hardware_executed.*PreparedButNotExecuted" source/ || true
```

Empty output expected. See `source/RULES.md` section 6B "One lifecycle state
field per object."

---

## File-Size Informational Check

Optional. Reports line counts for situational awareness only. Must not be used
to mechanically split, truncate, or delete code. See `source/RULES.md` §25A.

```bash
find source -path source/target -prune -o -type f -name '*.rs' -print | xargs wc -l | sort -n | tail -20
```

Thresholds are review triggers: 800 lines (review), 1200 lines (justify new
behavior), 1500 lines (decomposition plan required). Do not split files solely
because they appear in this list.

---

## Module Layout Structure Checks

Optional. Informational only. Must not cause automatic deletion or refactoring.

```bash
# Gateway-file audit: mod.rs files should not exist without justification
find source -path source/target -prune -o -type f -name 'mod.rs' -print

# Dumping-ground audit: these names should not appear without narrow owner scope
find source -path source/target -prune -o -type f \( -name 'types.rs' -o -name 'utils.rs' -o -name 'common.rs' -o -name 'helpers.rs' \) -print
```

See `source/RULES.md` §27A and §25C. Hits are review triggers, not errors.

---

## Cleanup Policy

Before marking a stage complete per `source/ROADMAP.md`:
- Remove any temporary gates that the stage introduced.
- Remove any `None` placeholder production steps reserved for future hooks.
- Remove any enum variants that have no construction site in production code.
- Collapse duplicate policy layers if one mechanism suffices.
- Verify no forbidden patterns were introduced.

---

## QEMU Rule

QEMU smoke tests and interactive runs are allowed **only when explicitly
requested**. Never run QEMU as part of a validation sweep unless the user asks.

Typical QEMU invocation (for reference only — do not run without request):

```bash
# RISC-V
qemu-system-riscv64 -machine virt -kernel kernel-rv -nographic

# LoongArch
qemu-system-loongarch64 -machine virt -kernel kernel-la -nographic
```

---

## Official Evaluation Rule

Run full official evaluation **only when explicitly requested**:

```bash
timeout 30m bash /home/lenovo/oscomp-official-env/run_official_autotest.sh \
  /home/lenovo/oscomp-official-env \
  /home/lenovo/projects/uestc-kernel
```

After evaluation, inspect the newest log:

```text
/home/lenovo/oscomp-official-env/logs/evaluate_*
```

Never claim a score unless the full official workflow completed and produced an
inspectable verdict/score/rank.

---

## Scan Exclusions

All validation greps must exclude build artifacts where possible:

```bash
# Exclude target directories
grep ... source/ --exclude-dir=target || true

# Exclude reference/archive trees
# source.1/target must not be scanned
```

`source.1/` is archived reference; its contents are not subject to `source/`
validation rules.
