# 简明 zCore 教程

[简明 zCore 教程](README.md)
[zCore 整体结构和设计模式](zcore-intro.md)
[Fuchsia OS 和 Zircon 微内核](src/fuchsia.md)  
[Fuchsia 安全原理](src/fuchsia-sec.md)

- [内核对象](src/ch01-00-object.md)
    - [初识内核对象](src/ch01-01-kernel-object.md)
    - [对象管理器：Process 对象](src/ch01-02-process-object.md)
    - [对象传送器：Channel 对象](src/ch01-03-channel-object.md)

- [任务管理](src/ch02-00-task.md)
    - [Zircon 任务管理体系](src/ch02-01-zircon-task.md)
    - [进程管理：Process 与 Job 对象](src/ch02-02-process-job-object.md)
    - [线程管理：Thread 对象](src/ch02-03-thread-object.md)

- [内存管理](src/ch03-00-memory.md)
    - [Zircon 内存管理模型](src/ch03-01-zircon-memory.md)
    - [物理内存：VMO 对象](src/ch03-02-vmo.md)
    - [物理内存：按页分配的 VMO](src/ch03-03-vmo-paged.md)
    - [虚拟内存：VMAR 对象](src/ch03-04-vmar.md)

- [用户程序](src/ch04-00-userspace.md)
    - [Zircon 用户程序](src/ch04-01-user-program.md)
    - [加载 ELF 文件](src/ch04-02-load-elf.md)
    - [上下文切换](src/ch04-03-context-switch.md)
    - [Zircon 系统调用](src/ch04-04-syscall.md)

- [信号和等待](src/ch05-00-signal-and-waiting.md)
    - [等待内核对象的信号](src/ch05-01-wait-signal.md)
    - [同时等待多个信号：Port 对象](src/ch05-02-port-object.md)
    - [实现更多：EventPair, Timer 对象](src/ch05-03-more-signal-objects.md)
    - [用户态同步互斥：Futex 对象](src/ch05-04-futex-object.md)