# IO_STATUS

## v73 - vector/range I/O scaffold

Implemented:
- readv/writev return 0 for zero-length smoke calls
- pread64/pwrite64 scaffold
- preadv/pwritev scaffold
- sendfile scaffold
- vmsplice/splice/tee scaffold
- copy_file_range scaffold

Still TODO:
- real iovec walking and copyin/copyout
- real offset updates
- pipe-backed splice
- file-backed range copy
- integration with real fd table and VFS
