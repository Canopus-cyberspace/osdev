# NET_STATUS

## v72 - socket/network scaffold

Implemented:
- socket returns fixed fd 11
- socketpair writes fixed fds 12 and 13
- bind/listen/connect/shutdown return 0
- accept/accept4 returns fixed fd 14 and writes a loopback sockaddr
- getsockname/getpeername write a loopback sockaddr
- sendto returns the requested length
- recvfrom returns 0 bytes
- setsockopt/getsockopt return 0, getsockopt writes optval=1 and optlen=4

Still TODO:
- real socket object table
- TCP/UDP state machines
- packet buffers
- blocking readiness
- network device driver
