# LAST_APPLIED_FIX

## v44 - Sv39 U-mode preparation scaffold

Status: PASS expected after package smoke test.

### What changed

- Added `src/mm/user_sv39.rs`.
- Added user text / guard / stack mapping plan metadata.
- Added permission checks for future Sv39 U-mode mapping.
- Kept the default runtime path on the already-passing kernel Sv39 trap smoke.
- Did not restore Sv39 + U-mode yet.

### Verified markers

- `[sv39-trap-v43e] after satp`
- `[sv39-trap-v43e] kernel trap smoke passed`

### Next planned step

v45:
- Create a controlled Sv39 + U-mode experiment branch.
- Map user text and user stack with real page tables.
- Enter U-mode only after kernel trap path is stable.
