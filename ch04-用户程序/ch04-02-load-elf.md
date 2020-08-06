Editing...
# [zCore程序(ELF加载与动态链接)](https://fuchsia.dev/fuchsia-src/concepts/booting/program_loading)

zCore内核不直接参与正常程序的加载，而是提供了一些用户态程序加载时可用的模块。如虚拟内存对象(VMO)、进程(processes)和线程(threads)这些。 


### ELF 格式以及系统应用程序二进制接口(system ABI)


标准的zCore用户空间环境提供了动态链接器以及基于ELF的执行环境，能够运行ELF格式的格式机器码可执行文件。zCore进程只能通过zCore vDSO使用系统调用。内核采用基于ELF系统常见的程序二进制接口(ABI)提供了vDSO。  

具备适当功能的用户空间代码可通过系统调用直接创建进程和加载程序，而不用ELF。但是zCore的标准ABI使用了这里所述的ELF。有关ELF文件格式的背景知识如下： 

### ELF文件类型 
“ET_REL”代表此ELF文件为可重定位文件  

“ET_EXEC“代表ELF文件为可执行文件  
 
“ET_DYN”代表此ELF文件为动态链接库  

“ET_CORE”代表此ELF文件是核心转储文件  


### 传统ELF程序文件加载  

可执行链接格式(Executable and Linking Format, ELF)最初由 UNIX 系统实验室开发并发布，并成为大多数类Unix系统的通用标准可执行文件格式。在这些系统中，内核使用```POSIX```(可移植操作系统接口)```execve API```将程序加载与文件系统访问集成在一起。该类系统加载ELF程序的方式会有一些不同，但大多遵循以下模式:  


1. 内核按照名称加载文件，并检查它是ELF还是系统支持的其他类型的文件。  


2. 内核根据ELF文件的```PT_LOAD```程序头来映射ELF映像。对于```ET_EXEC```文件，系统会将程序中的各段(Section)放到```p_vaddr```中所指定内存中的固定地址。对于```ET_DYN```文件，系统将加载程序第一个```PT_LOAD```的基地址，然后根据它们的```p_vaddr```相对于第一个section的```p_vaddr```放置后面的section。 通常来说该基地址是通过地址随机化(ASLR)来产生的。  


3. 若ELF文件中有一个```PT_INTERP```(Program interpreter)程序头,  它的部分内容(ELF文件中```p_offset```和```p_filesz```给出的一些字节)被当做为一个文件名，改文件名用于寻找另一个称为“ELF解释器”的ELF文件。上述这种ELF文件是```ET_DYN```文件。内核采用同样的方式将该类ELF文件加载，但是所加载的地址是自定的。该ELF“解释器”通常指的是被命名为```/lib/ld.so.1``` 或者是 ```/lib/ld-linux.so.2```的ELF动态链接器。



4. 内核为初始的线程设置了寄存器和堆栈的内容，并在PC寄存器已指向特定程序入口处(Entry Point)的情况下启动线程。 
    + 程序入口处(Entry Point)指的是ELF文件头中 ```e_entry```的值，它会根据程序基地址(base address)做相应的调整。如果这是一个带有```PT_INTERP```的ELF文件，则它的入口点不在它本身，而是被设置在动态链接器中。
    + 内核通过设置寄存器和堆栈来使得程序能够接收特定的参数，环境变量以及其它有实际用途的辅助向量。寄存器和堆栈的设置方法遵循了一种汇编级别的协议方式。若ELF文件运行时依赖动态链接，即带有```PT_INTERP```。则寄存器和堆栈中将包括来自该可执行文件的ELF文件头中的基地址、入口点和程序头部表地址信息，这些信息可允许动态链接器在内存中找到该可执行文件的ELF动态链接元数据，以实现动态链接。当动态链接启动完成后，动态链接器将跳转到该可执行文件的入口点地址。

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
zCore的程序加载受到了传统方式的启发，但是有一些不同。在传统模式中，需要在加载动态链接器之前加载可执行文件的一个关键原因是，动态链接器随机化选择的基地址(base address)不能与```ET_EXEC```可执行文件使用的固定地址相交。zCore从根本上并不支持```ET_EXEC```格式ELF文件的固定地址程序加载，它只支持位置无关的可执行文件或[PIE](https://patchwork.kernel.org/patch/9807325/)(```ET_DYN```格式的ELF文件)


### VmarExt trait实现 

zCore底层的API不支持文件系统。zCore程序文件的加载通过虚拟内存对象(VMO)以及```channel```使用的进程间通信机制来完成。

程序的加载基于如下一些前提：
+ 获得一个包含可执行文件的虚拟内存对象（VMO）的句柄。

> zircon-object\src\util\elf_loader.rs
```shell
fn make_vmo(elf: &ElfFile, ph: ProgramHeader) -> ZxResult<Arc<VmObject>> {
    assert_eq!(ph.get_type().unwrap(), Type::Load);
    let page_offset = ph.virtual_addr() as usize % PAGE_SIZE;
    let pages = pages(ph.mem_size() as usize + page_offset);
    let vmo = VmObject::new_paged(pages);
    let data = match ph.get_data(&elf).unwrap() {
        SegmentData::Undefined(data) => data,
        _ => return Err(ZxError::INVALID_ARGS),
    };
    vmo.write(page_offset, data)?;
    Ok(vmo)
}
```
+ 程序执行参数列表。
+ 程序执行环境变量列表。
+ 存在一个初始的句柄列表，每个句柄都有一个句柄信息项。


### zCore所支持的三种


### 标准zCore ELF动态链接器

