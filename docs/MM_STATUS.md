# MM_STATUS

## v76 - memory/policy/fd scaffold

Implemented:
- mremap returns 0 for zero-length smoke calls
- msync/mlock/munlock/mlockall/munlockall return 0
- mincore returns 0 and can zero a vector byte
- remap_file_pages returns 0
- mbind/get_mempolicy/set_mempolicy return 0
- memfd_create returns fixed fd 21
- userfaultfd returns fixed fd 22

Still TODO:
- real VMA splitting/merging
- real page residency accounting
- real memory locking accounting
- NUMA policy support
- real anonymous file objects for memfd
- real userfaultfd event delivery
