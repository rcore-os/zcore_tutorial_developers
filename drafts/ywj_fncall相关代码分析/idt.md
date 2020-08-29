```
use alloc::boxed::Box;
use x86_64::structures::idt::*;
use x86_64::structures::DescriptorTablePointer;
use x86_64::PrivilegeLevel;
```

#### 中断描述符表的结构

| Entry | description |
| - | - |
| `pub divide_by_zero: Entry<HandlerFunc>` | 	A divide by zero exception (`#DE`) occurs when the denominator of a DIV instruction or an IDIV instruction is 0. A `#DE` also occurs if the result is too large to be represented in the destination. | 
|`pub debug: Entry<HandlerFunc>` | <br> Instruction execution. <br>Instruction single stepping. <br>Data read. <br>Data write. <br>I/O read. <br>I/O write. <br>Task switch. <br>Debug-register access, or general detect fault" 
| `pub non_maskable_interrupt: Entry<HandlerFunc>` |	An non maskable interrupt exception (NMI) occurs as a result of system logic signaling a non-maskable interrupt to the processor. |
| `pub breakpoint: Entry<HandlerFunc>` | 	A breakpoint (`#BP`) exception occurs when an `INT3` instruction is executed.
| `pub overflow: Entry<HandlerFunc>` |	An overflow exception (`#OF`) occurs as a result of executing an `INTO` instruction while the overflow bit in `RFLAGS` is set to 1.
| `pub bound_range_exceeded: Entry<HandlerFunc>` |	A bound-range exception (`#BR`) exception can occur as a result of executing the `BOUND` instruction.
| `pub invalid_opcode: Entry<HandlerFunc>` |	An invalid opcode exception (`#UD`) occurs when an attempt is made to execute an  invalid or undefined opcode.
| `pub device_not_available: Entry<HandlerFunc>` |	"A device not available exception (`#NM`) occurs under any of the following conditions:     <br>- An `FWAIT`/`WAIT` instruction is executed when `CR0.MP=1` and `CR0.TS=1`.     <br>- Any x87 instruction other than `FWAIT` is executed when `CR0.EM=1`.   <br>- Any x87 instruction is executed when `CR0.TS=1`. The `CR0.MP` bit controls whether the  `FWAIT`/`WAIT` instruction causes an `#NM` exception when `TS=1`.     <br>- Any 128-bit or 64-bit media instruction when `CR0.TS=1`."
| `pub double_fault: Entry<HandlerFuncWithErrCode>`|	A double fault (`#DF`) exception can occur when a second exception occurs during the handling of a prior (first) exception or interrupt handler.
|`coprocessor_segment_overrun: Entry<HandlerFunc>`|	This interrupt vector is reserved. It is for a discontinued exception originally used by processors that supported external x87-instruction coprocessors.
|`pub invalid_tss: Entry<HandlerFuncWithErrCode>`|	An invalid TSS exception (`#TS`) occurs only as a result of a control transfer through a gate descriptor that results in an invalid stack-segment reference using an `SS` selector in the TSS.
|`pub segment_not_present: Entry<HandlerFuncWithErrCode>`|	An segment-not-present exception (`#NP`) occurs when an attempt is made to load a segment or gate with a clear present bit.
| `pub stack_segment_fault: Entry<HandlerFuncWithErrCode>`| "An stack segment exception (`#SS`) can occur in the following situations: <br> - Implied stack references in which the stack address is not in canonical form. Implied stack references include all push and pop instructions, and any instruction using `RSP` or `RBP` as a base register. <br> - Attempting to load a stack-segment selector that references a segment descriptor containing a clear present bit. <br> - Any stack access that fails the stack-limit check."
|`pub general_protection_fault: Entry<HandlerFuncWithErrCode>`|	"A general protection fault (`#GP`) can occur in various situations. Common causes include: <br>- Executing a privileged instruction while `CPL > 0`. <br>- Writing a 1 into any register field that is reserved, must be zero (MBZ). <br>- Attempting to execute an SSE instruction specifying an unaligned memory operand. <br>- Loading a non-canonical base address into the `GDTR` or `IDTR`. <br>- Using WRMSR to write a read-only MSR. <br>- Any long-mode consistency-check violation."
| `pub page_fault: Entry<PageFaultHandlerFunc>`|	"A page fault (`#PF`) can occur during a memory access in any of the following situations: <br>- A page-translation-table entry or physical page involved in translating the memory access is not present in physical memory. This is indicated by a cleared present bit in the translation-table entry. <br>- An attempt is made by the processor to load the instruction TLB with a translation for a non-executable page. <br>- The memory access fails the paging-protection checks (user/supervisor, read/write, or both). <br>- A reserved bit in one of the page-translation-table entries is set to 1. A `#PF` occurs for this reason only when `CR4.PSE=1` or `CR4.PAE=1`."
|`reserved_1: Entry<HandlerFunc>`|	reserved
|`pub x87_floating_point: Entry<HandlerFunc>`|	The x87 Floating-Point Exception-Pending exception (`#MF`) is used to handle unmasked x87 floating-point exceptions.
|`pub alignment_check: Entry<HandlerFuncWithErrCode>`|	An alignment check exception (`#AC`) occurs when an unaligned-memory data reference is performed while alignment checking is enabled.
|`pub machine_check: Entry<HandlerFunc>`|	The machine check exception (`#MC`) is model specific. Processor implementations are not required to support the `#MC` exception, and those implementations that do support `#MC` can vary in how the `#MC` exception mechanism works.
| `pub simd_floating_point: Entry<HandlerFunc>`|	"The SIMD Floating-Point Exception (`#XF`) is used to handle unmasked SSE floating-point exceptions. The SSE floating-point exceptions reported by the `#XF` exception are (including mnemonics): <br>- IE: Invalid-operation exception (also called #I). <br>- DE: Denormalized-operand exception (also called #D). <br>- ZE: Zero-divide exception (also called #Z). <br>- OE: Overflow exception (also called #O). <br>- UE: Underflow exception (also called #U). <br>- PE: Precision exception (also called #P or inexact-result exception)."
| `pub virtualization: Entry<HandlerFunc>` |	virtualization
| `reserved_2: [Entry<HandlerFunc>; 9]`|	reserved
| `reserved_2: [Entry<HandlerFunc>; 9]`|	reserved
| `reserved_2: [Entry<HandlerFunc>; 9]`|	reserved
| `reserved_2: [Entry<HandlerFunc>; 9]`|	reserved
| `reserved_2: [Entry<HandlerFunc>; 9]`|	reserved
| `reserved_2: [Entry<HandlerFunc>; 9]`|	reserved
| `reserved_2: [Entry<HandlerFunc>; 9]`|	reserved
| `reserved_2: [Entry<HandlerFunc>; 9]`|	reserved
| `reserved_2: [Entry<HandlerFunc>; 9]`|	reserved
| `pub security_exception: Entry<HandlerFuncWithErrCode>`|	The Security Exception (`#SX`) signals security-sensitive events that occur while executing the VMM, in the form of an exception so that the VMM may take appropriate action.
| `interrupts: [Entry<HandlerFunc>; 256 - 32]` | interupts


#### Entry 的结构
![IDT_Entry](/docs/riscv_doc/IDT_Entry.png)


#### New：Creates a new IDT filled with non-present entries.
![IDT_Entry_Missing](/docs/riscv_doc/IDT_Entry_Missing.png)

#### 初始化函数
```
pub fn init() {
    extern "C" {

    /// 引用汇编程序vector.S(由build.rs生成)中的中断向量表
        #[link_name = "__vectors"]
    /// 申请一个VECTOR, 由256个C函数指针组成, 函数指针指向vector.S中的中断向量表
        static VECTORS: [extern "C" fn(); 256];
    }


    /// 新建中断描述符表(filled with non-present entries)
    let idt = Box::leak(Box::new(InterruptDescriptorTable::new()));


    /// 申请一个entries，然后从idt中把内容transmute_copy过来，
    /// transmute_copy 的说明：pub unsafe fn transmute_copy<T, U>(src: &T) -> U
    /// Interprets src as having type &U, and then reads src without moving the contained value.

    let entries: &'static mut [Entry<HandlerFunc>; 256] =
        unsafe { core::mem::transmute_copy(&idt) };

    /// 将VECTORS中的函数指针写入对应的entries中, 并设置存在位
    /// Set the handler address for the IDT entry and sets the present bit. 
    /// 其中, 中断3, 4与其他的区别在于优先级为最低, 见figure_3和figure_4
    for i in 0..256 {
        let opt = entries[i].set_handler_fn(unsafe { core::mem::transmute(VECTORS[i]) });
        // Enable user space `int3` and `into`
        if i == 3 || i == 4 {
            opt.set_privilege_level(PrivilegeLevel::Ring3);
        }
    }

    /// load idt
    idt.load();
}
```

#### 中断3,4与其他中断

中断3,4的entrys的示意图
![Figure_3](/docs/riscv_doc/IDT_Entry_34.png)

其他中断的entrys的示意图
![Figure_4](/docs/riscv_doc/IDT_Entry_other.png)


#### 获取当前IDT寄存器中的数据

```
/// Get current IDT register
#[allow(dead_code)]
#[inline]

/// function sidt(): Get current IDT register; return: DescriptorTablePointer
fn sidt() -> DescriptorTablePointer {
    let mut dtp = DescriptorTablePointer { limit: 0, base: 0 };
    unsafe {
        asm!("sidt [{}]", in(reg) &mut dtp);
    }
    dtp
}
```
