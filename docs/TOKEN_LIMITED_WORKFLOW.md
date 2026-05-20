# Token-Limited Codex Workflow

## Principle

Use the web chat for long-range planning and failure analysis.

Use Codex for reading code, editing files, building, and running QEMU.

## Codex should receive short prompts

Each Codex prompt should contain only:

- current baseline
- one version goal
- preserved markers
- forbidden rules
- build/QEMU commands
- final PASS condition

Do not paste the full historical roadmap every time.

`AGENTS.md` and the docs in this package should hold the stable long-term rules.

## When Codex is interrupted

Use:

```text
Continue from current working tree.
Do not reset/revert/stash.
First inspect git status --short, git diff --stat, git diff.
Then continue from the existing partial implementation.
```

## When a build fails

Do not broaden scope.

Ask Codex to fix only the current compiler error and rerun build.

## When a marker is missing

Ask Codex to inspect only the fresh QEMU serial log.

Do not accept marker strings from source files, repair logs, guard logs, or build logs.

## When a version passes

Record:

- version
- changed files
- implemented semantics
- preserved markers
- new marker
- build log
- QEMU log
- remaining TODO

Then update the baseline for the next version.
