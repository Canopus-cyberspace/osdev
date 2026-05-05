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

## v61 - mmap/munmap scaffold

Implemented:
- fixed mmap window at `0x40040000..0x40044000`
- mapped mmap pages as R/W/U
- `mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0)` returns `0x40040000`
- `munmap(0x40040000, 4096)` clears scaffold active state and returns 0

Still TODO:
- real VMA list
- MAP_FIXED handling
- file-backed mmap
- page fault based lazy mapping
- real PTE unmap and TLB invalidation per VMA

## v62 - mprotect/madvise scaffold

Implemented:
- `mprotect(mmap_addr, 4096, PROT_READ)` validates fixed mmap window and returns 0
- `madvise(mmap_addr, 4096, MADV_NORMAL)` validates fixed mmap window and returns 0

Still TODO:
- real PTE permission update
- VMA metadata
- lazy mmap
- page fault handling
