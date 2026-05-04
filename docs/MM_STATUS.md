# MM_STATUS

## Completed

- Frame allocator basic test passed.
- Page table map/translate test passed.
- User-copy abstraction passed through v32e.
- UserAddressSpace metadata scaffold passed through v33.
- Sv39 pure dry-run passed through v34f.
- Kernel address-space dry-run passed through v35e/v35f.
- Safe Sv39 activation scaffold passed through v36e.
- Kernel address-space builder dry-run added in v37.

## Current Constraint

- Sv39 is still not enabled.
- `satp` is not written by the v37 path.
- Current U-mode still runs in no-Sv39 physical-address mode.
