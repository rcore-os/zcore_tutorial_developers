# zcore_tutorial_developers
zcore_tutorial文档撰写工作以及单元测试工作组

# 主要相关链接
https://github.com/rcore-os/zCore （zCore 仓库）  
https://rcore-os.github.io/zCore/zircon_object （zCore 代码文档）  
https://fuchsia.dev/fuchsia-src/reference （Zircon 官方文档）  

https://raw.githubusercontent.com/wiki/rcore-os/zCore/files/wrj-thesis.pdf（wrj，Rust语言操作系统的设计与实现）
https://raw.githubusercontent.com/wiki/rcore-os/zCore/files/pql-thesis.pdf（pql，zCore操作系统内核的设计与实现）

# 现有的zcore_tutorial文档主要从这些方面展开描述

1. 内核对象
1.1. 初识内核对象   
1.2. 对象管理器：Process 对象       <zircon-object\src\task\process.rs>  job/process/thread  
1.3. 对象传送器：Channel 对象       <zircon-object\src\ipc\channel.rs>  
2. 任务管理                         <zircon-object\src\task>  
2.1. Zircon 任务管理体系            <>  
2.2. 硬件抽象层与``异步运行时``      <kernel_hal(bare)> async 《zCore 操作系统内核的设计与实现》中有相关描述  
2.3. 线程管理：Thread 对象          <zircon-object\src\task\thread.rs>std::thread(8.4日)  
2.4. 进程管理：Process 与 Job 对象  <zircon-object\src\task\job.rs>  <zircon-object\src\task\job_policy.rs>  
3. 内存管理  
3.1. Zircon 内存管理模型  
3.2. 物理内存：VMO 对象             <zircon-object\src\vm\vmo\physical.rs>   
3.3. 虚拟内存：VMAR 对象            <zircon-object\src\vm\vmar.rs>  
4. 用户程序
4.1. Zircon 用户程序                
4.2. 加载 ELF 文件                  <zircon-object\src\util\elf_loader.rs>  
4.3. 上下文切换                     
4.4. 系统调用                       <zircon-syscall\src>

# zCore项目整理架构图
![file](http://www.nuanyun.cloud/wp-content/uploads/2020/08/5f2a17fc7d7b3.png)