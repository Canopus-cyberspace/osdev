# PROCESS_STATUS

## v52 - execve-oriented process initialization batch

Status: scaffold implemented

Implemented:
- `loader::process_image::UserProgram`
- `loader::process_image::ProcessInitInfo`
- init PID/PPID placeholders
- argv0 placeholder
- envp count placeholder
- auxv placeholder
- user stack bottom/top metadata
- page table root placeholder
- `loader::user_stack::InitialUserStackLayout`
- initial user stack dry-run with:
  - argc
  - argv[0]
  - argv NULL
  - envp NULL
  - auxv placeholders: AT_PHDR, AT_PHENT, AT_PHNUM, AT_PAGESZ, AT_ENTRY, AT_NULL

Not implemented yet:
- real process table
- real thread object
- scheduler integration
- real `execve` syscall
- writing the dry-run stack into the actual runtime user stack
- loading binaries from VFS/rootfs instead of `include_bytes!`
