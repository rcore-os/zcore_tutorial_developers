# [对象管理器：Process 对象（关于进程的抽象）](https://fuchsia.dev/fuchsia-src/reference/kernel_objects/process)
  
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
#[allow(dead_code)]
pub struct Process {
    /// 内核对象核心结构，定义于 zircon-object/src/object/mod.rs 
    base: KObjectBase,
    /// 引用计数？ 定义于 zircon-object/src/object/mod.rs
    _counter: CountHelper,
    /// 属于的作业，定义于 zircon-object/src/task/job.rs
    job: Arc<Job>,
    /// 
    policy: JobPolicy,
    vmar: Arc<VmAddressRegion>,
    ext: Box<dyn Any + Send + Sync>,
    exceptionate: Arc<Exceptionate>,
    debug_exceptionate: Arc<Exceptionate>,
    inner: Mutex<ProcessInner>,
}
```

## 句柄和权限

## 实现第一个内核对象

## 存储内核对象句柄

## 根据句柄查找内核对象