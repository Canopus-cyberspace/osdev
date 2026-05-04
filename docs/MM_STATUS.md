# MM_STATUS

## Completed

- v31 user-copy abstraction
- v32e user-copy bounds and getppid test
- v33 user address-space metadata scaffold
- v34f pure Sv39 dry-run
- v35 kernel address-space dry-run
- v36e safe Sv39 activation scaffold
- v37 kernel mapping builder dry-run
- v38 user mapping builder dry-run
- v39 real page table build dry-run
- v40d Sv39 smoke scaffold
- v42 isolated kernel-only Sv39 activation smoke

## Current Notes

- v42 writes satp in a kernel-only isolated path.
- v42 does not enter U-mode.
- The next step should restore trap/U-mode under Sv39 only after kernel Sv39 is stable.
