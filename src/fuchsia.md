# Fuchsia OS 和 Zircon 微内核

## Fuchsia
[开发 Fuchsia 的目的是什么](https://www.digitaltrends.com/mobile/google-fuchsia-os-news/)

Fuchsia 是谷歌试图使用单一操作系统去统一整个生态圈的一种尝试，Fuchsia 的目标是能够在谷歌的技术保护伞下，运行于智能手机、智能音响、笔记本电脑等任何合适的设备之上。据某消息人士透露，谷歌计划在未来三年内，先让 Fuchsia 在智能音响和其他智能家具设备上运行起来，然后再转移到笔记本电脑等更大的设备上，并最终取代 Android 成为世界上最大的移动操作系统。

## Fuchsia OS
Fuchsia是一款开源操作系统，它的安全性和可更新性都得到了充分的考虑。

#### Fuchsia的安全性

Fuchsia主推安全和隐私。软件是在密封的包中交付的，所有的东西都是沙箱（Sandbox），这意味着系统上运行的所有软件，包括应用程序和系统组件，都能获得执行其工作所需的最低权限，并且只能访问它需要知道的信息。
Fuchsia OS的核心并不基于Linux（区别于Android），取而代之的是Zircon。

#### Fuchsia的可更新性
Fuchsia的工作方式是将组件打包交付。Fuchsia软件包被设计为独立更新，甚至是临时交付，这意味着软件包被设计为在需要时从设备进出，软件总是最新的，就像一个网页。Fuchsia的目标是为驱动程序提供二进制稳定的接口。在未来，为一个版本的Fuchsia编译的驱动程序将继续在Fuchsia的未来版本中工作，而不需要修改甚至重新编译。这意味着Fuchsia设备将能够在保持现有驱动程序的同时无缝地更新到最新版本的Fuchsia。


## Zircon Kernel
Fuchsia不使用Linux内核。相反，Fuchsia有自己的内核，Zircon，它是从微内核演变而来的。Fuchsia将POSIX规范的一部分(但不是全部)作为底层内核原语之上的库实现，这些原语侧重于安全消息传递和内存管理。许多核心系统服务，比如文件系统和网络。这些服务在内核之外运行时尽量满足最小特权原则，如提供沙箱隔离机制。

Zircon内核提供系统调用来管理进程、线程、虚拟内存、进程间通信、等待对象状态变化和锁定(通过futexes)。

如下是一些重要的Zircon内核模块
#### 任务管理: Jobs, Processes 和 Threads.
线程代表在一个地址空间中执行的线程(CPU寄存器、堆栈等)，这个地址空间是由它们存在的进程所拥有的。进程由作业拥有，作业定义了各种资源限制。Job属于父Job，一直到根Job，根Job是内核在引导时创建的，并被传递到userboot(第一个开始执行的用户空间进程)。如果没有Job句柄，进程中的线程就不可能创建另一个进程或另一个Job。

#### 信息传递机制
Message Passing: Sockets and Channels

#### 对象与信号
Objects and Signals

#### 虚拟内存对象
虚拟内存对象（VMO）表示内存的一组物理页面，或者潜在的页面(这些页面将按需惰性地创建/填充)。
VMOs也可以通过sys_vmo_read()和sys_vmo_write()直接读取和写入。因此，对于“创建VMO，将数据集写入其中，并将其交给另一个进程使用”这样的一次性操作，可以避免将它们映射到地址空间的成本。

#### 内存管理机制 
Address Space Management

####
Futexes

## 参考资料：
1. Fuchsia Overview：
https://fuchsia.dev/fuchsia-src/concepts

2. 开发 Fuchsia 的目的是什么？
https://www.digitaltrends.com/mobile/google-fuchsia-os-news/


## Fuchsia Partner

ARM
GlobalEdge Software
Huawei
Imagination Technologies
MediaTek
Oppo
Qualcomm
Samsung
Sharp
Sony
STMicro
Unisoc
Xiaomi