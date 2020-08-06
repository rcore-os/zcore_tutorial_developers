Editing...
# [zCore程序(ELF加载与动态链接](https://fuchsia.dev/fuchsia-src/concepts/booting/program_loading)

zCore内核不直接参与正常程序的加载，而是提供了一些用户态程序加载时可用的模块。如虚拟内存对象(VMO)、进程(processes)和线程(threads)这些。 


### ELF 格式以及系统应用程序二进制接口(system ABI)


标准的zCore用户空间环境提供了动态链接器以及基于ELF的执行环境，能够运行ELF格式的格式机器码可执行文件。zCore进程只能通过zCore vDSO使用系统调用。内核采用基于ELF系统常见的程序二进制接口(ABI)提供了vDSO。  

具备适当功能的用户空间代码可通过系统调用直接创建进程和加载程序，而不用ELF。但是zCore的标准ABI使用了这里所述的ELF。有关ELF文件格式的背景知识如下： 


### 传统ELF程序文件加载  

可执行链接格式(Executable and Linking Format, ELF)最初由 UNIX 系统实验室开发并发布，并成为大多数类Unix系统的通用标准可执行文件格式。在这些系统中，内核使用```POSIX```(可移植操作系统接口)```execve API```将程序加载与文件系统访问集成在一起。该类系统加载ELF程序的方式会有一些不同，但大多遵循以下模式:  


1. 内核按照名称加载文件，并检查它是ELF还是系统支持的其他类型的文件。  


2. 内核根据ELF文件的```PT_LOAD```程序头来映射ELF映像。对于```ET_EXEC```文件，系统会将程序中的各段(Section)放到```p_vaddr```中所指定内存中的固定地址。对于```ET_DYN```文件，系统将加载程序第一个```PT_LOAD```的基地址，然后根据它们的```p_vaddr```相对于第一个section的```p_vaddr```放置后面的section。   


3. 若ELF文件中有一个PT_INTERP程序头..  

4. 内核为初始的线程设置了寄存器和堆栈的内容，并在PC寄存器已指向特定入口点处的情况下启动线程。  
Todo:  
+ Entry Point的确定方式？  
+ 内核如何设置寄存器和堆栈？  

```rust
    pub fn sys_process_start(
        &self,
        proc_handle: HandleValue,
        thread_handle: HandleValue,
        entry: usize,
        stack: usize,
        arg1_handle: HandleValue,
        arg2: usize,
    ) -> ZxResult {
        info!("process.start: proc_handle={:?}, thread_handle={:?}, entry={:?}, stack={:?}, arg1_handle={:?}, arg2={:?}",
            proc_handle, thread_handle, entry, stack, arg1_handle, arg2
        );
        let proc = self.thread.proc();
        let process = proc.get_object_with_rights::<Process>(proc_handle, Rights::WRITE)?;
        let thread = proc.get_object_with_rights::<Thread>(thread_handle, Rights::WRITE)?;
        if !Arc::ptr_eq(&thread.proc(), &process) {
            return Err(ZxError::ACCESS_DENIED);
        }
        let arg1 = if arg1_handle != INVALID_HANDLE {
            let arg1 = proc.remove_handle(arg1_handle)?;
            if !arg1.rights.contains(Rights::TRANSFER) {
                return Err(ZxError::ACCESS_DENIED);
            }
            Some(arg1)
        } else {
            None
        };
        process.start(&thread, entry, stack, arg1, arg2, self.spawn_fn)?;
        Ok(())
    }
```

### Launchpad 库



## 
> Filesystems are not part of the lower layers of Zircon API. Instead, program loading is based on VMOs and on IPC protocols used through channels.


### 标准zCore ELF动态链接器