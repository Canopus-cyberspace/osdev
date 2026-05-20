# Official Environment Update Checklist

## Record current environment

```bash
ENV_ROOT=/home/lenovo/oscomp-official-env
AUTO="$ENV_ROOT/autotest-for-oskernel"
DATA="$ENV_ROOT/testdata"

git -C "$AUTO" rev-parse HEAD
git -C "$AUTO" fetch --all --prune
git -C "$AUTO" rev-parse origin/main
grep -nE 'os-contest:[0-9]+' "$AUTO/README.md" || true
```

## Docker images

```bash
docker image inspect zhouzhouyi/os-contest:20260104 --format '20260104 {{.Id}} {{.RepoDigests}}' 2>/dev/null || true
docker image inspect zhouzhouyi/os-contest:20260510 --format '20260510 {{.Id}} {{.RepoDigests}}' 2>/dev/null || docker pull zhouzhouyi/os-contest:20260510
```

## Judge hashes

```bash
sha256sum "$AUTO"/kernel/judge/judge_basic-musl.py \
          "$AUTO"/kernel/parse_output_2023.py \
          "$AUTO"/kernel/sdcardwork.py 2>/dev/null || true
```

## SDCARD raw hashes

```bash
gzip -dc "$DATA/sdcard-rv.img.gz" | sha256sum
gzip -dc "$DATA/sdcard-la.img.gz" | sha256sum
```

## Optional fresh image comparison

```bash
mkdir -p /tmp/oscomp_image_check
cd /tmp/oscomp_image_check

curl -fL --retry 5 --retry-delay 5 --retry-all-errors \
  -o sdcard-rv.img.xz.new \
  "https://github.com/oscomp/testsuits-for-oskernel/releases/download/pre-20250615/sdcard-rv.img.xz"

echo "local rv raw:"
gzip -dc /home/lenovo/oscomp-official-env/testdata/sdcard-rv.img.gz | sha256sum

echo "new rv raw:"
xz -dc /tmp/oscomp_image_check/sdcard-rv.img.xz.new | sha256sum
```
