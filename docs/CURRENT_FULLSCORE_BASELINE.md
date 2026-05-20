# Current Full-Score Baseline

## Baseline

```text
v194
+ official make all compatibility
+ official RISC-V QEMU clean shutdown
+ official basic-musl-rv score 2.0
```

## Confirmed

- Official environment setup works.
- Official Docker image works.
- Official sdcard images exist.
- `make all` works under official compile stage.
- `kernel-rv` boots under official QEMU and exits cleanly.
- Official scoring parser sees at least one valid content-backed `basic-musl` group.
- `basic-musl-rv` score is nonzero.

## Limitations

- `score: 2` is only the first nonzero milestone.
- `kernel-la` is not a real LoongArch64 kernel.
- Most official suites remain zero.
- Full-score work must follow official judge feedback.

## Immediate next target

Expand `basic-musl-rv` beyond score 2.0.
