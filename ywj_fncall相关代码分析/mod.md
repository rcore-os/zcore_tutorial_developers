#### mod.rs
#### **使用的一些macro和包，以后再详细写
```
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod fncall;
#[cfg(any(target_os = "none", target_os = "uefi"))]
mod gdt;
#[cfg(any(target_os = "none", target_os = "uefi"))]
mod idt;
#[cfg(feature = "ioport_bitmap")]
#[cfg(any(target_os = "none", target_os = "uefi"))]
pub mod ioport;
#[cfg(any(target_os = "none", target_os = "uefi"))]
mod syscall;
#[cfg(any(target_os = "none", target_os = "uefi"))]
mod trap;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use fncall::syscall_fn_entry;
#[cfg(any(target_os = "none", target_os = "uefi"))]
pub use trap::TrapFrame;
```
#### 初始化函数 
`pub unsafe fn init()`, 在x86_64架构下初始化中断处理

这个函数分为以下步骤:
1. 关闭x86_64中自带的中断功能  `x86_64::instructions::interrupts::disable();`

2. 初始化全局描述符表GDT(在GDT的分析中会详述它的实现)  `gdt::init();`
  - Switch to a new [GDT], extend 7 more entries from the current one. 
  - Switch to a new [TSS], set `GSBASE` to its base address.
  
  

3. 初始化中断描述符表IDT(在IDT的分析中会详述它的实现)  `idt::init();`
  - Switch to a new [IDT], override the current one.
  
  

4. 使能系统调用
  - Enable [`syscall`] instruction.
     - set `EFER::SYSTEM_CALL_EXTENSIONS`

[GDT]: https://wiki.osdev.org/GDT
[IDT]: https://wiki.osdev.org/IDT
[TSS]: https://wiki.osdev.org/Task_State_Segment
[`syscall`]: https://www.felixcloutier.com/x86/syscall

##### 完整代码如下：
```
#[cfg(any(target_os = "none", target_os = "uefi"))]
pub unsafe fn init() {
    x86_64::instructions::interrupts::disable();
    gdt::init();
    idt::init();
    syscall::init();
}
```

#### 定义一个表示用户上下文的结构体
该结构体包含通用寄存器，陷阱号，错误代码

```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct UserContext {
    pub general: GeneralRegs,
    pub trap_num: usize,
    pub error_code: usize,
}
```

为`UserContext`添加一些必要的方法

`get_syscall_num(&self)`: 从rax中获取系统调用号

`get_syscall_ret(&self)`: 从rax中获取系统调用返回值

`set_syscall_ret(&mut self, ret: usize)`: 将系统调用返回值写入rax

`get_syscall_args(&self)`: 获取系统调用的参数

`set_ip(&mut self, ip: usize)`: 将指令寄存器的值设置为参数指定的地址

`set_sp(&mut self, sp: usize)`: 将栈指针寄存器的值设置为参数指定的地址

`get_sp(&self)`: 获取栈指针

`set_tls(&mut self, tls: usize)`: 将Thread Local Storage(TLS)写入fsbase

```
impl UserContext {
    /// Get number of syscall
    pub fn get_syscall_num(&self) -> usize {
        self.general.rax
    }

    /// Get return value of syscall
    pub fn get_syscall_ret(&self) -> usize {
        self.general.rax
    }

    /// Set return value of syscall
    pub fn set_syscall_ret(&mut self, ret: usize) {
        self.general.rax = ret;
    }

    /// Get syscall args
    pub fn get_syscall_args(&self) -> [usize; 6] {
        [
            self.general.rdi,
            self.general.rsi,
            self.general.rdx,
            self.general.r10,
            self.general.r8,
            self.general.r9,
        ]
    }

    /// Set instruction pointer
    pub fn set_ip(&mut self, ip: usize) {
        self.general.rip = ip;
    }

    /// Set stack pointer
    pub fn set_sp(&mut self, sp: usize) {
        self.general.rsp = sp;
    }

    /// Get stack pointer
    pub fn get_sp(&self) -> usize {
        self.general.rsp
    }

    /// Set tls pointer
    pub fn set_tls(&mut self, tls: usize) {
        self.general.fsbase = tls;
    }
}
```

#### 通用寄存器

```
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct GeneralRegs {
    pub rax: usize, 
    pub rbx: usize,
    pub rcx: usize,
    pub rdx: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rbp: usize,
    pub rsp: usize,
    pub r8: usize,
    pub r9: usize,
    pub r10: usize,
    pub r11: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
    pub rip: usize,
    pub rflags: usize,
    pub fsbase: usize,
    pub gsbase: usize,
}
```

寄存器|功能
---|---
rax|函数返回值
rbx|用作数据存储，遵循被调用者使用规则
rcx|用作函数参数，第四个参数
rdx|用作函数参数，第三个参数
rsi|用作函数参数，第二个参数
rdi|用作函数参数，第一个参数
rbp|用作数据存储，遵循被调用者使用规则
rsp|栈指针寄存器，指向栈顶
r8|用作函数参数，第五个参数
r9|用作函数参数，第六个参数
r10|用作数据存储，遵循调用者使用规则
r11|用作数据存储，遵循调用者使用规则
r12|用作数据存储，遵循被调用者使用规则
r13|用作数据存储，遵循被调用者使用规则
r14|用作数据存储，遵循被调用者使用规则
r15|用作数据存储，遵循被调用者使用规则
rip|指令寄存器
rflags|64位标志寄存器
[fsbase](https://wiki.osdev.org/SWAPGS)|Their base addresses are used to calculate effective addresses. FS is used for Thread Local Storage. 
[gsbase](https://wiki.osdev.org/SWAPGS)|Their base addresses are used to calculate effective addresses. The GS register often holds a base address to a structure containing per-CPU data.

