# MM_STATUS

## v60 - brk heap scaffold

Implemented:
- fixed user heap window at `0x40030000..0x40034000`
- mapped heap pages as R/W/U
- `USER_BRK` starts at `USER_HEAP_START`
- `brk(0)` returns current break
- `brk(addr)` updates break when addr is inside the heap window
- invalid brk requests keep and return current break

Still TODO:
- lazy allocation
- dynamic heap expansion
- page fault based allocation
- mmap/munmap
- per-process address space ownership
