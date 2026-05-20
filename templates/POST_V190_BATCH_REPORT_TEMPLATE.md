# <BATCH> Report

## Baseline

`<BASELINE>`

## Batch goal

`<GOAL>`

## Files changed

- `<file>`

## Real behavior implemented

Describe actual runtime/kernel behavior. Do not only list markers.

## Live path integration

Explain how this connects to the live path, or document the new real user execution path if this batch adds one.

## Preserved markers

List all preserved markers from previous baseline.

## New markers

List new markers.

## Fresh QEMU evidence

```text
<qemu serial log path>
```

## Build log

```text
<build log path>
```

## Forbidden warning gate

PASS / FAIL.

## Remaining incomplete semantics

- `<remaining item>`

## Final output

```text
[PASS] <batch pass line>
[PASS] apply_fix.sh completed
[PASS] apply_fix.bat completed
```
