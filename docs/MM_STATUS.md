# MM_STATUS

## Completed

- Frame allocator test passes.
- Page table map/translate test passes.
- User copy abstraction passes direct and U-mode syscall tests.
- User address-space metadata scaffold passes.
- v34f: Sv39 pure dry-run preflight passes without allocating physical frames or disturbing U-mode.

## Current Constraints

- Sv39 is not enabled yet.
- satp is not written yet.
- User program still runs in physical-address no-Sv39 smoke mode.
- ELF loader is not implemented yet.

## Next

- Introduce kernel identity mapping preflight.
- Introduce user image mapping plan.
- Only then enable Sv39 in a narrow test branch.
