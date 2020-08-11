# 对象管理器：Process 对象

Zircon的进程是传统意义上程序的实例：由一个或多个线程执行的一组指令以及相关的资源集合组成。   

进程对象是以下资源的容器集合：
* [句柄（Handles）](https://fuchsia.dev/fuchsia-src/concepts/kernel/handles)
* [虚拟内存地址区域（VMAR）](https://fuchsia.dev/fuchsia-src/reference/kernel_objects/vm_address_region)
* [线程（Threads）](https://fuchsia.dev/fuchsia-src/reference/kernel_objects/thread)   

通常，它与正在执行的代码相关联，直到强制终止或程序退出为止。
进程由[作业（Jobs）](https://fuchsia.dev/fuchsia-src/reference/kernel_objects/job)所拥有。每个进程属于一个作业，并且从资源和权限限制以及生命周期的控制的角度上看，由多个进程组成的应用程序被视为单个实体。  

进程的定义如下：
```rust
// zircon-object/src/task/process.rs
// 进程
#[allow(dead_code)]
pub struct Process {
    /// 内核对象核心结构，定义于 zircon-object/src/object/mod.rs 
    base: KObjectBase,
    /// 引用计数，定义于 zircon-object/src/object/mod.rs
    _counter: CountHelper,
    /// 属于的作业，定义于 zircon-object/src/task/job.rs
    job: Arc<Job>,
    /// policy，定义于 zircon-object/src/task/job_policy.rs
    policy: JobPolicy,
    /// VMAR，定义于 zircon-object/src/vm/vmar.rs
    vmar: Arc<VmAddressRegion>,
    ext: Box<dyn Any + Send + Sync>,
    /// Exceptionate(Kernel-owned exception channel endpoint)，定义于 zircon-object/src/task/exception.rs
    exceptionate: Arc<Exceptionate>,
    debug_exceptionate: Arc<Exceptionate>,
    /// 进程的内部可变部分
    inner: Mutex<ProcessInner>,
}

// 进程的内部可变部分
#[derive(Default)]
struct ProcessInner {
    /// 进程的状态
    status: Status,
    max_handle_id: u32,
    /// 句柄(Handle)，定义于 zircon-object/src/object/handle.rs
    handles: HashMap<HandleValue, (Handle, Vec<Sender<()>>)>,
    /// Futex(A primitive for creating userspace synchronization tools)，定义于
    futexes: HashMap<usize, Arc<Futex>>,
    threads: Vec<Arc<Thread>>,

    // special info
    debug_addr: usize,
    dyn_break_on_load: usize,
    critical_to_job: Option<(Arc<Job>, bool)>,
}

// 进程的状态
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Init,
    Running,
    Exited(i64),
}
```


## 句柄和权限

[句柄]: https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/handles.md
[权限]: https://github.com/zhangpf/fuchsia-docs-zh-CN/blob/master/zircon/docs/rights.md

> 介绍并实现 Handle，Rights

## 实现第一个内核对象

> 使用上一节的方法，实现一个空的 Process 对象

## 存储内核对象句柄

> 添加成员变量 handles: BTreeMap<HandleValue, Handle>
>
> 实现 create，add_handle，remove_handle 函数

## 根据句柄查找内核对象

> 实现 get_object_with_rights 等其它相关函数
>
> 实现 handle 单元测试
