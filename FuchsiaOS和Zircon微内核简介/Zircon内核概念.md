# 介绍

Zircon内核管理许多不同类型的对象。本质上讲，这些对象是能够通过系统调用直接访问，并实现了Dispatcher接口的C++类。对象的实现位于[kernel object](https://fuchsia.dev/fuchsia-src/reference/kernel_objects/objects)目录下。

 

# 内核对象ID

Zircon内核为了管理这些对象，就需要给他们打上标签。

在内核中的每个对象都具有"内核对象id"或简称为“koid"。它们是64位无符号整型，用以唯一定位这些对象，并且在系统生命周期内是唯一的，这尤其意味着koid将不会被重用。

有两种特别的koid值：

*ZX_KOID_INVALID*的值为0，可用做表示”null“的哨兵值。 :question::question::question:这里两个的还需解释要改进一下

*ZX_KOID_KERNEL*只有一个内核，并且具有它自己的koid :question::question::question:



# 句柄(Handle)

Zircon是面向对象的内核, 在用户态下的代码几乎完全通过对象句柄(`object handls`)与OS资源交互。

即: `Handle`（句柄）是Zircon内核的“文件描述符”，它表示用户空间进程如何引用内核对象，通过`Channel`可以将`Handle`传递给其他进程.

对象可能有多个句柄（在一个或多个进程中）引用它们。对于所有的`Object`，当最后指向它们的`Handle`关闭时，该`Object`同时也将被销毁，或者放置到结束状态下，并且这样的操作不可回滚。

可以通过向`Channel`写入`Handle`的方式，将`Handle`从一个`Process`移动到另外一个`Process`（使用 *zx_channel_write()*函数），或通过使用*zx_process_start()*函数，作为新`Process`的第一个线程的启动参数的形式传递一个`Handle`。

在某种程度上可以将句柄理解为指针, 在对句柄或者其指向对象进行操作时受到`Right`(权限)的控制, 就好像在用户态不能执行某些特权级指令一样。并且在指向对象的数个句柄中，`Right`级别可以不一样。





# 系统调用（System calls）

用户空间代码通过系统调用与内核对象进行交互，以及几乎只能通过句柄对象（`Handle`）加以访问。在用户空间中，`Handle`通过32位整型（`zx_handle_t`类型)来表示。 当系统调用被执行时，内核检查`Handle`变量指向的实际句柄是否存在于调用进程的句柄列表中。然后内核进一步检查该`Handle`是否具有正确的类型（例如传递一个Thread Handle到需要事件信号的系统调用会导致错误发生），以及`Handle`是否对请求的操作具有相应的权限（`Right`）。



从访问的角度上看，系统调动可以被细分为如下三大类：

1. 没有任何限制的调用。只有少部分调用是属于这一类，如*zx_clock_get()*和*zx_nanosleep()*可以被任意线程调用。
2. 以`Handle`作为首参数的调用，用以表示它们所操作的对象。绝大多数的调用都是这一类，例如*zx_channel_write()*和*zx_port_queue()*。
3. 创建新对象的系统调用，不需要以`Handle`作为参数。例如*zx_event_create()*和*zx_channel_create()*。这一类调用的访问是受调用`Process`进程所在的`Job`所控制的（这同时也是它们的限制）。



# 运行代码：任务（Job），进程（Process）和线程（Thread）

在上面提到了`Process`进程所在的`Job`, 现在解释一下什么是`Job,Process,Thread`。

在rCore中已经有了`Process`和`Thread`的概念。`Thread`表示在拥有它们的`Process`的地址空间中线程的执行（CPU寄存器，运行栈等）。`Process`被`Job`所拥有，而后者定义了各种可用资源的上限。`Job`又被父`Job`所拥有，并一直可以追溯到根任务（`Root Job`）。根任务于内核在启动时所创建，并被传递到第一个被执行的用户进程`userboot`中。

> userboot是Zircon内核启动的第一个进程。 它以与vDSO相同的方式从内核映像加载，而不是从文件系统加载。 其主要目的是从bootfs加载第二个process和bootsvc。

离开了指向`Job`的`Handle`，`Process`中的`Thread`将无法创建其他`Process`或`Job`。





# 消息传递：Socket和Channel

`Socket`和`Channel`都是双向和双端的进程间通信（IPC）相关的`Object`。创建`Socket`或`Channel`将返回两个不同的`Handle`，分别指向`Socket`或`Channel`的两端。

> **进程间通信**（**IPC**，*Inter-Process Communication*），指至少两个进程或线程间传送数据或信号的一些技术或方法。进程是计算机系统分配资源的最小单位(进程是分配资源最小的单位，而线程是调度的最小单位，线程共用进程资源)。每个进程都有自己的一部分独立的系统资源，彼此是隔离的。为了能使不同的进程互相访问资源并进行协调工作，才有了进程间通信。举一个典型的例子，使用进程间通信的两个应用可以被分类为客户端和服务器，客户端进程请求数据，服务端回复客户端的数据请求。有一些应用本身既是服务器又是客户端，这在分布式计算中，时常可以见到。这些进程可以运行在同一计算机上或网络连接的不同计算机上。

`Socket`是面向流的对象，可以通过它读取或写入以一个或多个字节为单位的数据。Short writes（如果Socket的缓冲区已满）和short read（如果请求的字节数量超过缓冲区大小）也同样受到支持。

`Channel`是面向数据包的对象，并限制消息的大小最多为64K（如果有改变，可能会更小），以及最多1024个`Handle`挂载到同一消息上（如果有改变，同样可能会更小）。无论消息是否满足short write或read的条件，`Channel`均不支持它们。

当`Handle`被写入到`Channel`中时，在发送端`Process`中将会移除这些`Handle`。同时携带`Handle`的消息从`Channel`中被读取时，该`Handle`也将被加入到接收端`Process`中。在这两个时间点之间时，`Handle`将同时存在于两端（以保证它们指向的`Object`继续存在而不被销毁），除非`Channel`写入方向一端被关闭，这种情况下，指向该端点的正在发送的消息将被丢弃，并且它们包含的任何句柄都将被关闭。

