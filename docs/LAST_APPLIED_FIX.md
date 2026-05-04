# LAST_APPLIED_FIX

Version: v38
Name: user mapping builder dry-run
Status: PASS expected after apply_fix

Summary:
- Added `src/mm/user_builder.rs`.
- Added `UserAddressSpaceBuilder` metadata dry-run.
- Validates text / guard / stack user regions.
- Does not enable Sv39.
- Does not write `satp`.
- Keeps U-mode syscall matrix as regression test.
