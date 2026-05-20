# Official Environment Update Compatibility Report

## Summary

The local official environment was updated from the older Docker baseline to the current public README baseline before finishing P01.

```text
old Docker tag: zhouzhouyi/os-contest:20260104
new Docker tag: zhouzhouyi/os-contest:20260510
autotest old HEAD: e54f7264ea112f18ceb15ad7c6131f9070bfea6e
autotest new HEAD: 500e7edcfb875409a0babe125d273ab30771d5ec
```

`git pull --ff-only origin main` completed cleanly. The upstream change updated README Docker references to `20260510`. `kernel.zip` was rebuilt from the updated `kernel/` directory.

## Local Wrapper Update

Updated local official environment files:

```text
/home/lenovo/oscomp-official-env/oscomp_official_env.env
/home/lenovo/oscomp-official-env/run_official_autotest.sh
```

Both now resolve the official Docker image as:

```text
zhouzhouyi/os-contest:20260510
```

The requested Windows wrapper path was absent in this WSL mount:

```text
/mnt/c/Users/lenovo/Downloads/oscomp_env_setup/v214_official_oscomp_env_setup_with_bat/scripts/run_official_autotest.sh
```

The environment-local wrapper was used:

```text
/home/lenovo/oscomp-official-env/run_official_autotest.sh
```

## Fingerprints

```text
Docker 20260510 digest: zhouzhouyi/os-contest@sha256:85dec949df7cef41fd03d30c6ad69f952204540e18d2c62bced9d2e262fef12d
Docker 20260104 digest before cleanup: zhouzhouyi/os-contest@sha256:5c04dbc38562b1cd578c33c9cd321d4731cb8cdd00c82b2320a4350754faa6b0
kernel.zip sha256: bfbb6d149f4bf2da120114fa92f16764a48b6b1754df2f4c92cd620bba47ebec
judge_basic-musl.py sha256: e53f62cb7444b4bdb2cd84014931516ae7cfd5b91595d541dfe901d048f9fc52
parse_output_2023.py sha256: 8d5fed9b330a94ce9522608bfb3bf9c0df99e9e651b5293561c86af8dc2bbfbb
sdcardwork.py sha256: 5ffbf5c48cfe0b34f9e18bf97461785fd493b9866a941755b2736e09b54c340f
```

The local sdcard xz/gz assets decompress to matching raw images:

```text
sdcard-rv raw sha256: 95973543db6b84a9a5e70f30da466ce292867aff5b689fb14c88dc9406e378b8
sdcard-la raw sha256: 1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50
```

Compressed asset hashes:

```text
sdcard-rv.img.xz: 2b59aee4d26681b2d78a8faa615ac216f723a8ea157544e7867bbc8c77aac087
sdcard-rv.img.gz: 9a98993b3142f7a7c2ed00a248cd092ab013a29533d2d052c8904ec4430876d5
sdcard-la.img.xz: 3cbeae1095d788cd8255e4929bcc6dace96dbe4999b6eaec003ae6fe2cdba42b
sdcard-la.img.gz: 59bacc01b33356123fd1ed9293dc839ec33f83b2b90f59ea79b52b2f91213db5
```

## Revalidation

The updated official Docker run completed:

```text
log: /home/lenovo/oscomp-official-env/logs/evaluate_20260513_001517/docker_evaluate.log
verdict: Accpted
score: 35
basic-musl-rv: 35.0
```

The preserved content-backed `test_write` path remains valid as part of the P01 group:

```text
test_write pass=2 score=2
```

## Cleanup

Cleanup was performed only after the updated official run passed.

Before cleanup:

```text
Docker images: 2
Docker image size: 52.3GB
Build cache: 0B
```

After cleanup:

```text
Docker images: 1
Docker image size: 26.82GB
Build cache: 0B
Removed old image: zhouzhouyi/os-contest:20260104
```

Commands used:

```text
docker container prune -f
docker builder prune -f
docker image rm zhouzhouyi/os-contest:20260104 || true
rm -f /home/lenovo/oscomp-official-env/testdata/*.part
rm -f /home/lenovo/oscomp-official-env/testdata/*.tmp
rm -f /home/lenovo/oscomp-official-env/testdata/sdcard-rv.img
rm -f /home/lenovo/oscomp-official-env/testdata/sdcard-la.img
```

The compressed official sdcard assets were preserved.

## Repair Log Policy

Small-evidence repair logging is installed in:

```text
tools/prune_repair_logs.sh
apply_fix.sh
```

The policy prevents `.repair_logs` from accumulating raw sdcard images, compressed sdcard releases, kernel binaries, ELF payloads, Cargo target trees, or copied repository/cache directories. The pruning tool records size, sha256, path, and file type before deleting bulky artifacts, gzips text logs larger than 1 MB, preserves named evidence directories unless they exceed 500 MB, and keeps only the latest three non-evidence repair run directories.
