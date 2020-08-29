# zcore_tutorial_developers
zcore_tutorial文档撰写工作以及单元测试工作组
## 主要目的
建立这个仓库的主要目的为尽量减少重复的工作。由于每个人的学习程度不一样，导致前面的人遇到的坑可能后人还会碰到。为了让前人的工作能给后人一定的帮助，建立此仓库为后人提供支持。

## 主要要求
每个人尽量把自己学到的东西写成一个文档，做到低耦合、规范化，命名清晰。后人在前人的基础上迭代修改，最后项目完成的时候进一步整理一下，可以merge到zcore仓库。




# 主要相关链接
[2020年操作系统专题训练大实验-zCore文档WiKi页面](http://os.cs.tsinghua.edu.cn/oscourse/OsTrain2020/g2)

[zCore仓库](https://github.com/rcore-os/zCore)

[zCore代码文档](https://rcore-os.github.io/zCore/zircon_object)

[zCore-Tutorial](https://github.com/rcore-os/zCore-Tutorial)

[Zircon 官方文档](https://fuchsia.dev/fuchsia-src/reference)

[wrj，Rust语言操作系统的设计与实现](https://raw.githubusercontent.com/wiki/rcore-os/zCore/files/wrj-thesis.pdf )

[pql，zCore操作系统内核的设计与实现](https://raw.githubusercontent.com/wiki/rcore-os/zCore/files/pql-thesis.pdf)

[Rust中的Async /Await](https://github.com/rustcc/writing-an-os-in-rust/blob/master/12-async-await.md)  

# 文件仓库目录描述

[初步文档仓库(本仓库)](https://github.com/rcore-os/zcore_tutorial_developers)  

[目标文档仓库(release版本的仓库)](https://github.com/rcore-os/zCore-Tutorial)

本仓库中的目录说明：  

- `src/chxx`文件夹分别对应每一章节的内容
- `help`目录是方便zCore程序分析的相关文档
- `drafts`是指等待被整理进入`src/chxx`文件夹中的内容
- `img`存放`src/chxx`文档中的所有图片,图表等
- `SUMMARY.md`是对zcore_tutorial文档中各个章节的索引目录  
  

# 现有的zcore_tutorial文档主要从这些方面展开描述

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
    - [上下文切换](drafts/ywj_fncall相关代码分析/mod.md)
    - [Zircon 系统调用](src/ch04-04-syscall.md)

- [信号和等待](src/ch05-00-signal-and-waiting.md)
    - [等待内核对象的信号](src/ch05-01-wait-signal.md)
    - [同时等待多个信号：Port 对象](src/ch05-02-port-object.md)
    - [实现更多：EventPair, Timer 对象](src/ch05-03-more-signal-objects.md)
    - [用户态同步互斥：Futex 对象](src/ch05-04-futex-object.md)

# zCore项目整理架构图
![file](http://www.nuanyun.cloud/wp-content/uploads/2020/08/5f2a17fc7d7b3.png)





