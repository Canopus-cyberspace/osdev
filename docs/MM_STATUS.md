# MM_STATUS

## Passed

- Frame allocator smoke test
- Page table map/translate test
- User copy abstraction and bounds path
- User address-space metadata scaffold
- Sv39 pure dry-run
- Kernel address-space dry-run
- Kernel/user mapping builder dry-run
- Real page table build dry-run
- Sv39 activation scaffold v40c

## Not Enabled Yet

- Writing `satp`
- Running with Sv39 enabled
- Page fault handling
- Real user address space switching

## v40d - Sv39 smoke compile fix

- `sv39::test_scaffold()` exists.
- `sv39_smoke::test()` exists.
- Sv39 activation remains disabled.
- U-mode syscall matrix must continue passing.
