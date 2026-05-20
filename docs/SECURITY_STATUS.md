# SECURITY_STATUS

## v74 - identity/session/capability scaffold

Implemented:
- capget/capset scaffolds
- personality scaffold
- uid/gid mutation scaffolds
- getresuid/getresgid write zero IDs
- setfsuid/setfsgid return old ID 0
- process group/session scaffolds
- getgroups/setgroups scaffolds
- getrlimit/setrlimit scaffolds
- getpriority/setpriority scaffolds
- times writes a zero tms structure

Still TODO:
- real credentials per task
- privilege checks
- capabilities bitsets and namespaces
- process group/session ownership
- real rlimit accounting
