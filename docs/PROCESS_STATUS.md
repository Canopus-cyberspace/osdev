# PROCESS_STATUS

## v51 - execve-oriented ProcessInitInfo scaffold

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
- self-test that validates external init ELF load metadata

Not implemented yet:
- real process table
- real thread object
- scheduler integration
- real `execve` syscall
- argv/envp/auxv writing onto user stack
- file-backed executable loading from VFS
