# 分析报告
## 概述
While the kernel handles the system call, the application's CPU state is saved in a trap frame on the thread's kernel stack, and the CPU registers are available to hold kernel execution state.

当内核处理系统调用时，应用程序的CPU状态保存在线程内核栈的trap frame中，CPU寄存器就能够进入内核状态

Zircon是一个可以适应多种指令集的内核，在src/arch下，有四个目录，分别对应四种不同的CPU结构，它们是:
 - aarch64 (ARM architecture)
 - mipsel (MIPS architecture)
 - riscv (RISC-V)
 - x86_64 (64 bit version of x86 instruction set)
## x86_64
### 文件
file | description
---- | ---
fncall.rs | 在同一个状态中，因函数调用造成的上下文切换的实现
gdt.rs |  配置全局描述符表(GDT)
idt.rs |  配置中断描述符表(IDT)
ioport.rs |  I/O端口Permission
mod.rs |  公共模块?
syscall.rs |  系统调用的入口(通过它进入syscall.S)(未完成)
syscall.S |  汇编代码，执行系统调用处理
trap.rs |  处理内核中断的入口(通过它进入trap.S)(未完成)
trap.S |  汇编代码，执行中断处理

### 代码分析
[mod.rs](/src/arch/x86_64/doc/mod.md)