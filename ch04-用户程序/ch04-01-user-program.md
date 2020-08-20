# Zircon 用户程序

## 用户态启动流程

### 流程概要
 kernel   
 -> userboot  (decompress bootsvc LZ4 format)   
 -> bootsvc   (可执行文件bin/component_manager)  
 -> component_manager   
 -> sh / device_manager  

### ZBI(Zircon Boot Image)
ZBI是一种简单的容器格式，它内嵌了许多可由引导加载程序 `BootLoader`传递的项目内容，包括硬件特定的信息、提供引导选项的内核“命令行”以及RAM磁盘映像(通常是被压缩的)。`ZBI`中包含了初始文件系统 `bootfs`，内核将 `ZBI` 完整传递给 `userboot`，由它负责解析并对其它进程提供文件服务。


### bootfs

基本的`bootfs`映像可满足用户空间程序运行需要的所有依赖:
+ 可执行文件
+ 共享库
+ 数据文件  
  
以上列出的内容还可实现设备驱动或更高级的文件系统，从而能够从存储设备或网络设备上访问读取更多的代码和数据。

在系统自引导结束后，`bootfs`中的文件就会成为一个挂载在根目录`/boot`上的只读文件系统树(并由bootsvc提供服务)。随后`userboot`将从`bootfs`加载第一个真正意义上的用户程序。



### USERBOOT

#### 使用userboot的原因 

在Zircon中，内嵌在ZBI中的`RAM磁盘映像`通常采用[LZ4](https://github.com/lz4/lz4)格式压缩。解压后将继续得到`bootfs`格式的磁盘镜像。这是一种简单的只读文件系统格式，它只列出文件名。且对于每个文件，可分别列出它们在BOOTFS映像中的偏移量和大小(这两个值都必须是页面对齐的，并且限制在32位)。

由于kernel中没有包含任何可用于解压缩[LZ4](https://github.com/lz4/lz4)格式的代码，也没有任何用于解析BOOTFS格式的代码。所有这些工作都是由称为`userboot`的第一个用户空间进程完成的。


> zCore中未找到解压缩bootfs的相关实现，  
> 但是能够在scripts/gen-prebuilt.sh中找到ZBI中确实有bootfs的内容  
> 且现有的zCore实现中有关所载入的ZBI方式如下：  

> zircon-loader/src/lib.rs
```rust
    // zbi
    let zbi_vmo = {
        let vmo = VmObject::new_paged(images.zbi.as_ref().len() / PAGE_SIZE + 1);
        vmo.write(0, images.zbi.as_ref()).unwrap();
        vmo.set_name("zbi");
        vmo
    };
```
#### userboot是什么
userboot是一个普通的用户空间进程。它只能像任何其他进程一样通过vDSO执行标准的系统调用，并受完整vDSO执行制度的约束。

userboot被构建为一个ELF动态共享对象(DSO,dynamic shared object)，使用了与vDSO相同的布局。与vDSO一样，userboot的ELF映像在编译时就被嵌入到内核中。其简单的布局意味着加载它不需要内核在引导时解析ELF的文件头。内核只需要知道三件事:
1. 只读段`segment`的大小
2. 可执行段`segment`的大小
3. `userboot`入口点的地址。  
   
这些值在编译时便可从userboot ELF映像中提取，并在内核代码中用作常量。

#### kernel如何启用userboot

与任何其他进程一样，userboot必须从已经映射到其地址空间的vDSO开始，这样它才能进行系统调用。内核将userboot和vDSO映射到第一个用户进程，然后在userboot的入口处启动它。

<!-- > !  userboot的特殊之处在于它的加载方式。   
> ...todo -->

#### userboot如何在vDSO中取得系统调用
当内核将`userboot`映射到第一个用户进程时，会像正常程序那样，在内存中选择一个随机地址进行加载。而在映射`userboot`的vDSO时，并不采用上述随机的方式，而是将vDSO映像直接放在内存中`userboot`的映像之后。这样一来，vDSO代码与`userboot`的偏移量总是固定的。

在编译阶段中，系统调用的入口点符号表会从vDSO ELF映像中提取出来，随后写入到链接脚本的符号定义中。利用每个符号在vDSO映像中相对固定的偏移地址，可在链接脚本提供的`_end`符号的固定偏移量处，定义该符号。通过这种方式，userboot代码可以直接调用到放在内存中，其映像本身之后的，每个确切位置上的vDSO入口点。

相关代码:
> zircon-loader/src/lib.rs
```rust
pub fn run_userboot(images: &Images<impl AsRef<[u8]>>, cmdline: &str) -> Arc<Process> {
    ...
    // vdso
    let vdso_vmo = {
        let elf = ElfFile::new(images.vdso.as_ref()).unwrap();
        let vdso_vmo = VmObject::new_paged(images.vdso.as_ref().len() / PAGE_SIZE + 1);
        vdso_vmo.write(0, images.vdso.as_ref()).unwrap();
        let size = elf.load_segment_size();
        let vmar = vmar
            .allocate_at(
                userboot_size,
                size,
                VmarFlags::CAN_MAP_RXW | VmarFlags::SPECIFIC,
                PAGE_SIZE,
            )
            .unwrap();
        vmar.map_from_elf(&elf, vdso_vmo.clone()).unwrap();
        #[cfg(feature = "std")]
        {
            let offset = elf
                .get_symbol_address("zcore_syscall_entry")
                .expect("failed to locate syscall entry") as usize;
            let syscall_entry = &(kernel_hal_unix::syscall_entry as usize).to_ne_bytes();
            // fill syscall entry x3
            vdso_vmo.write(offset, syscall_entry).unwrap();
            vdso_vmo.write(offset + 8, syscall_entry).unwrap();
            vdso_vmo.write(offset + 16, syscall_entry).unwrap();
        }
        vdso_vmo
    };
    ...

}
```

### bootsvc
bootsvc 通常是usermode加载的第一个程序（与userboot不同，userboot是由内核加载的）。bootsvc提供了几种系统服务：
+ 包含bootfs（/boot）内容的文件系统服务（初始的bootfs映像包含用户空间系统需要运行的所有内容:
  - 可执行文件
  - 共享库和数据文件（包括设备驱动程序或更高级的文件系统的实现）
+ 从bootfs加载的加载程序服务


### todo：  
+ bin/component_manager  
+ sh / device_manager    






## 用户程序的组成

> 内核不直接参与用户程序的加载工作（第一个进程除外）
>
> 用户程序强制使用 PIC 和 PIE（位置无关代码）
>
> 内存地址空间组成：Program, Stack, vDSO, Dylibs
>
> 通过 Channel 传递启动信息和句柄







## 加载 ELF 文件

> 简单介绍 ELF 文件的组成结构（[ch04-02-load-elf.md](（ch04-02-load-elf.md）)）
>
> 实现 VmarExt::load_from_elf 函数








## 系统调用的跳板：vDSO

#### 介绍 vDSO 的作用


vDSO（virtual Dynamic Shared Object），Zircon vDSO 是 Zircon 内核访问系统调用的唯一方法(作为系统调用的跳板)。它之所以是虚拟的，是因为它不是从文件系统中的ELF文件加载的，而是由内核直接提供的vDSO镜像。

Zircon vDSO是访问Zircon系统调用的唯一手段。vDSO表示虚拟动态共享对象。(动态共享对象是一个术语，用于ELF格式的共享库。)它是虚拟的，因为它不是从文件系统中的ELF文件加载的。相反，vDSO映像由内核直接提供。

> zCore/src/main.rs
```rust
#[cfg(feature = "zircon")]
fn main(ramfs_data: &[u8], cmdline: &str) {
    use zircon_loader::{run_userboot, Images};
    let images = Images::<&[u8]> {
        userboot: include_bytes!("../../prebuilt/zircon/x64/userboot.so"),
        vdso: include_bytes!("../../prebuilt/zircon/x64/libzircon.so"),
        zbi: ramfs_data,
    };
    let _proc = run_userboot(&images, cmdline);
    run();
}
```

它是一个用户态运行的代码，被封装成`prebuilt/zircon/x64/libzircon.so`文件。这个.so 文件装载不是放在文件系统中，而是由内核提供。它被整合在内核image中。

vDSO映像在编译时嵌入到内核中。内核将它作为只读VMO公开给用户空间。内核启动时，会通过计算得到它所在的物理页。当`program loader`设置了一个新进程后，使该进程能够进行系统调用的唯一方法是：`program loader`在新进程的第一个线程开始运行之前，将vDSO映射到新进程的虚拟地址空间（地址随机）。因此，在启动其他能够进行系统调用的进程的每个进程自己本身都必须能够访问vDSO的VMO。

> zircon-loader/src/lib.rs#line167  

```rust
    proc.start(&thread, entry, sp, Some(handle), 0, thread_fn)
        .expect("failed to start main thread");
    proc
```
> zircon-object/src/task/process.rs#line189  

```rust
    thread.start(entry, stack, handle_value as usize, arg2, thread_fn)
```

vDSO被映射到新进程的同时会将映像的`base address`通过`arg2`参数传递给新进程中的第一个线程。通过这个地址，可以在内存中找到ELF的文件头，该文件头指向可用于查找系统调用符号名的其他ELF程序模块。

#### 如何修改 vDSO 源码（libzircon）将 syscall 改为函数调用

有关代码
+ 参考仓库[README.MD](https://github.com/PanQL/zircon/blob/master/README.md)
    > ···解析代码依赖的compile_commands.json将会随build过程生成到**out**文件夹···
+ 配合zCore中的有关脚本与补丁文件
    - scripts/gen-prebuilt.sh
    - scripts/zircon-libos.patch
+ https://github.com/PanQL/zircon/blob/master/system/ulib/zircon/syscall-entry.h
+ https://github.com/PanQL/zircon/blob/master/system/ulib/zircon/syscalls-x86-64.S
+ zircon-loader/src/lib.rs#line 83-93
```rust

        #[cfg(feature = "std")]
        {
            let offset = elf
                .get_symbol_address("zcore_syscall_entry")
                .expect("failed to locate syscall entry") as usize;
            let syscall_entry = &(kernel_hal_unix::syscall_entry as usize).to_ne_bytes();
            // fill syscall entry x3
            vdso_vmo.write(offset, syscall_entry).unwrap();
            vdso_vmo.write(offset + 8, syscall_entry).unwrap();
            vdso_vmo.write(offset + 16, syscall_entry).unwrap();
        }

```

<!-- 当vsdo 用svc 指令后，这时CPU exception进入内核，到 expections.S 中的 sync_exception 宏（不同ELx， sync_exception的参数不一样）。然后这个 sync_exception 宏中先做一些现场保存的工作， 然后jump到 arm64_syscall_dispatcher 宏。

进入arm64_syscall_dispatcher宏后， 先做一些syscall number检查，然后syscall number 跳到 call_wrapper_table 函数表中相应index项的函数中去（call_wrapper_table 像一个一维的函数指针的数组，syscall number 作index，jump到相应的wrapper syscall function 函数中去）。 -->

#### 加载 vDSO 时修改 vDSO 代码段，填入跳转地址







## 第一个用户程序：userboot

> 实现 zircon-loader 中的 run_userboot 函数
> 
> 能够进入用户态并在第一个系统调用时跳转回来


#### 从`bootfs`加载第一个真正意义上的用户程序。
主要相关代码：
> zircon-loader/src/lib.rs
> zircon-object/src/util/elf_loader.rs

当`userboot`解压完毕`ZBI`中的`bootfs`后，`userboot`将继续从`bootfs`载入程序文件运行。

Zircon中具体的实现流程如下：
1. `userboot`检查从内核接收到的环境字符串，这些字符串代表了一定的内核命令行。
    > zircon-loader/src/main.rs
    ```rust
    #[async_std::main]
    async fn main() {
        kernel_hal_unix::init();
        init_logger();

        let opt = Opt::from_args();
        let images = open_images(&opt.prebuilt_path).expect("failed to read file");

        let proc: Arc<dyn KernelObject> = run_userboot(&images, &opt.cmdline);
        drop(images);

        proc.wait_signal(Signal::USER_SIGNAL_0).await;
    }
    ```
   在Zircon中：
   + 若该字符串内容为```userboot=file```，那么该`file`将作为第一个真正的用户进程加载。
   + 若没有这样的选项，则`userboot`将选择的默认文为`bin/bootsvc`。该文件可在`bootfs`中找到。
  
   而在zCore的实现中：
   + ..
2. 为了加载上述文件，userboot实现了一个功能齐全的ELF程序加载器
   `zircon_object::util::elf_loader::load_from_elf`
    ```rust
        // userboot
        let (entry, userboot_size) = {
            let elf = ElfFile::new(images.userboot.as_ref()).unwrap();
            let size = elf.load_segment_size();
            let vmar = vmar
                .allocate(None, size, VmarFlags::CAN_MAP_RXW, PAGE_SIZE)
                .unwrap();
            vmar.load_from_elf(&elf).unwrap();
            (vmar.addr() + elf.header.pt2.entry_point() as usize, size)
        };
    ```
3. 然后userboot以随机地址加载vDSO。它使用标准约定启动新进程，并给它传递一个channel句柄和vDSO基址。
   `zircon_object::util::elf_loader::map_from_elf`
