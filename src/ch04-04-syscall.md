# Target:
# 系统调用

## 获取系统调用参数

> 从寄存器中获取参数

## 系统调用上下文与处理函数

> 定义 Syscall 结构体，实现 syscall 函数

## 实现第一个系统调用

> 实现 sys_channel_read 和 sys_debuglog_write






----
# 系统调用
> [package]
name = "zircon-syscall"


## 系统调用上下文与处理函数

> 定义 Syscall 结构体，实现 syscall 函数

## 实现第一个系统调用

> 实现 sys_channel_read 和 sys_debuglog_write





## 系统调用简要流程说明

### 构造 Syscall 结构体
> 保存上下文信息

```rust
pub struct Syscall<'a> {
    pub regs: &'a mut GeneralRegs,
    pub thread: Arc<Thread>,
    pub spawn_fn: fn(thread: Arc<Thread>),
    pub exit: bool,
}
```

### 调用syscall（），
```rust
pub async fn syscall(&mut self, num: u32, args: [usize; 8]) -> isize 
```

### 获得系统调用号
```rust
let sys_type = match Sys::try_from(num) {
```



### 获取系统调用参数

> 从寄存器中获取参数
```rust
let [a0, a1, a2, a3, a4, a5, a6, a7] = args;
```




### 根据`sys_type` 匹配对应的系统调用
```rust
let ret = match sys_type {

```

> 系统调用号的对应关系保存在const.rs中




### 简单实现一个syscall(sys_clock_adjust为例子)
```rust
    pub fn sys_clock_adjust(&self, resource: HandleValue, clock_id: u32, offset: u64) -> ZxResult {
    // 1. 记录log信息：info!()
        info!(
            "clock.adjust: resource={:#x?}, id={:#x}, offset={:#x}",
            resource, clock_id, offset
        );
    // 2. 检查参数合法性（需要归纳出每个系统调用的参数值的范围）
    // missing

    // 3. 获取当前进程对象
        let proc = self.thread.proc();
    // 4. 根据句柄从进程中获取对象
        proc.get_object::<Resource>(resource)?
            .validate(ResourceKind::ROOT)?;
        match clock_id {
            ZX_CLOCK_MONOTONIC => Err(ZxError::ACCESS_DENIED),
    // 5. 调用内河对象API执行具体功能
            ZX_CLOCK_UTC => {
                UTC_OFFSET.store(offset, Ordering::Relaxed);
                Ok(())
            }
            _ => Err(ZxError::INVALID_ARGS),
        }
    }
```


> 不完全的实现

```rust
    /// Acquire the current time.  
    ///   
    /// + Returns the current time of clock_id via `time`.  
    /// + Returns whether `clock_id` was valid.  
    pub fn sys_clock_get(&self, clock_id: u32, mut time: UserOutPtr<u64>) -> ZxResult {
        // 记录log信息：info!()
        info!("clock.get: id={}", clock_id); 
        // 检查参数合法性
        // miss

        match clock_id {
            ZX_CLOCK_MONOTONIC => {
                time.write(timer_now().as_nanos() as u64)?;
                Ok(())
            }
            ZX_CLOCK_UTC => {
                time.write(timer_now().as_nanos() as u64 + UTC_OFFSET.load(Ordering::Relaxed))?;
                Ok(())
            }
            ZX_CLOCK_THREAD => {
                time.write(self.thread.get_time())?;
                Ok(())
            }
            _ => Err(ZxError::NOT_SUPPORTED),
        }
    }
```


