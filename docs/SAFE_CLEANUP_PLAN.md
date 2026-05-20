# Safe Cleanup Plan

Only clean after the 20260510 environment is pulled and the current basic-musl-rv baseline is revalidated.

## Guarded cleanup

```bash
docker container prune -f
docker builder prune -f
docker image rm zhouzhouyi/os-contest:20260104 || true

rm -f /home/lenovo/oscomp-official-env/testdata/*.part
rm -f /home/lenovo/oscomp-official-env/testdata/*.tmp
rm -f /home/lenovo/oscomp-official-env/testdata/sdcard-rv.img
rm -f /home/lenovo/oscomp-official-env/testdata/sdcard-la.img
```

## Do not delete

```text
sdcard-rv.img.gz
sdcard-la.img.gz
fresh docker_evaluate.log
fresh console_log
fresh os_serial_out_rv.txt
fresh .repair_logs evidence
```

## Record before/after

```bash
df -h /
docker system df
du -h --max-depth=2 /home/lenovo/oscomp-official-env 2>/dev/null | sort -h | tail -30
```
