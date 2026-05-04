# SV39_STATUS

## Completed

- v34f: pure non-destructive user Sv39 dry-run.
- v35: kernel address-space dry-run.
- v36e/v40d: Sv39 activation scaffold.
- v42: kernel-only Sv39 activation smoke passed.
- v43e: kernel Sv39 trap smoke with `ebreak` passed.
- v44: U-mode under Sv39 mapping plan scaffold added.

## Current default test mode

- Kernel-only Sv39 trap smoke remains the default safe path.
- U-mode under Sv39 is not enabled yet.

## Next

- v45 should map user text / user stack under Sv39 and run a controlled U-mode ecall experiment.
