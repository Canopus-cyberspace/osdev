# MM_STATUS

## Passed

- Frame allocator smoke test
- Basic page table map / translate test
- User copy abstraction and bounds checks
- User address-space metadata scaffold
- Sv39 pure dry-run preflight
- Kernel address-space dry-run
- Sv39 activation scaffold without writing `satp`
- Kernel mapping builder dry-run
- User mapping builder dry-run
- v39 real `AddressSpace` page-table build dry-run without activating `satp`

## Current constraints

- Sv39 is not enabled yet.
- `satp` is not written in the normal path.
- ELF loader is still stub.
- Real user address-space switch is not implemented yet.
