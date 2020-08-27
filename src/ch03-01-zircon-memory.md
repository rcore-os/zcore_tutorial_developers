# Zircon 内存管理模型

## Zircon内存使用的3种方式:
A process can use memory 3 ways:
+ 以堆、线程栈、可执行代码+数据的形式映射内存。这个内存由`VMARs`表示，而`VMARs`又持有一个对`VMOs`的引用。程序员通常通过内存地址与内存进行交互 
+ 独立的`VMOs`。这些是没有通过`VMAR`映射的内存页集。程序员通过句柄与内存进行交互（通常利用`vmo_read`和`vmo_write`的API）
+ 以内核对象句柄形式存在的内核内存







