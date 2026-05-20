# v207-v214 Storage Image Report

## Scope

- Bounded batch: v207-v214 only.
- Stable baseline: v206.
- Preserved fresh QEMU runtime markers from v151k7 through v206.
- Intentionally not implemented in this batch: LoongArch64, GUI, full TCP/IP, dynamic kernel modules, shell, and BusyBox.

## v207 Block Device Abstraction

- Added canonical runtime block-device state with a read/write sector interface.
- The device records sector size, capacity, read count, write count, and error count.
- Invalid block reads return stable `EINVAL` and increment device error accounting.

Fresh runtime evidence:

```text
[ucompat-v207] evidence devices=1 sector=512 capacity=24 reads=1 writes=7 errors=1 PASS
[ucompat-v207] block device abstraction PASS
```

## v208 Ramdisk Backend

- Added a ramdisk backend behind the block-device interface.
- The image is constructed through block writes, then read back through block reads.
- The trait-shaped block path is ready for a future virtio-blk backend, but this batch intentionally uses ramdisk to keep risk bounded.

Fresh runtime evidence:

```text
[ucompat-v208] evidence backend=ramdisk sector=512 capacity=24 readback=18 PASS
[ucompat-v208] virtio blk ramdisk backend PASS
```

## v209 Block Cache

- Added a deterministic block cache with fixed slots.
- Cache reads track hits and misses.
- Cache writes mark dirty entries and flush them back through the block-device write path.

Fresh runtime evidence:

```text
[ucompat-v209] evidence hits=1 misses=2 dirty=1 writebacks=1 PASS
[ucompat-v209] block cache PASS
```

## v210 Readonly Image Mount

- Implemented a bounded readonly filesystem-image parser mounted at `/image`.
- The parser reads a ramdisk image header and directory-entry table through the block cache.
- It installs image-backed VFS nodes with inode number, image offset, size, mode, and executable metadata.
- This is not a full Ext4 implementation. Unsupported Ext4 features include journals, extents beyond the bounded table, xattrs, checksums, block groups, symlinks, hard links, and writeback.

Fresh runtime evidence:

```text
[ucompat-v210] evidence mount=/image dirs=3 files=5 execs=3 metadata_reads=2 PASS
[ucompat-v210] ext4 readonly mount PASS
```

## v211 Execve From Filesystem Image

- `execve_from_vfs` now reads ELF bytes through a shared node-file helper.
- Rootfs files still use in-memory node bytes.
- Imagefs files read through the block cache and ramdisk block device.
- ELF validation and exec VMA setup therefore work for `/image/bin/*.elf` without copying those bytes into internal rootfs storage.

Fresh runtime evidence:

```text
[ucompat-v211] evidence path=/image/bin/hello.elf entry=0x40000000 phnum=1 image_reads=2 PASS
[ucompat-v211] execve from fs image PASS
```

## v212 Image Metadata I/O

- `stat`, `getdents`, `open`, and `read` work on image-backed nodes.
- Directory reads are backed by canonical VFS nodes created from parsed image metadata.
- File reads are backed by image offsets and block-cache reads.

Fresh runtime evidence:

```text
[ucompat-v212] evidence stat_size=24 dents=80 readme=24 elf_magic=ELF PASS
[ucompat-v212] fs image metadata io PASS
```

## v213 Image Rootfs Compatibility Matrix

- The image contains multiple files and programs:
  - `/image/README.txt`
  - `/image/etc/config.txt`
  - `/image/bin/hello.elf`
  - `/image/bin/worker.elf`
  - `/image/bin/tool.elf`
- The runtime matrix discovers, reads, and executes multiple ELF files through the image-backed VFS path and collects expected exit statuses.

Fresh runtime evidence:

```text
[ucompat-v213] evidence programs=3 worker_status=7936 tool_status=9472 config_len=33 PASS
[ucompat-v213] image rootfs compatibility matrix PASS
```

## v214 Filesystem Submission Hardening

- Stable image-path error behavior is covered:
  - `ENOENT` for missing image path.
  - `ENOTDIR` for traversal through a regular file.
  - `EISDIR` for opening a directory as read/write.
  - `EINVAL` for unsupported filesystem type.
  - `EBADF` for invalid fd.
  - `EACCES` for readonly image write open.

Fresh runtime evidence:

```text
[ucompat-v214] evidence ENOENT=-2 ENOTDIR=-20 EISDIR=-21 EINVAL=-22 EBADF=-9 EACCES=-13 PASS
[ucompat-v214] filesystem submission hardening PASS
```

## Verification

Commands run:

```bash
cargo build --target riscv64gc-unknown-none-elf
bash ./tools/run-qemu.sh
```

Fresh QEMU serial log:

```text
.repair_logs/qemu-run-20260509_201406.serial.log
```

Result:

- PASS: all v151k7-v214 markers present in the fresh QEMU serial log.
- PASS: v207-v214 markers are backed by block-device, cache, image mount, image file I/O, image exec, and errno evidence.
- PASS: build output did not contain `matches any value`, `unreachable pattern`, or `warning: unused variable:`.

