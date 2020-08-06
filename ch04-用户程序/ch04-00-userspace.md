# 用户程序


zCore has a microkernel style of design. A complexity for microkernel designs is how to bootstrap the initial userspace processes. Often this is accomplished by having the kernel implement minimal versions of filesystem reading and program loading just for the purpose of bootstrapping, even when those kernel facilities are never used after boot time. zCore takes a different approach.