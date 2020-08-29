#### gdt.rs的分析
#### TODO: 使用的一些macro和包，以后详细写
```
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem::size_of;

use x86_64::instructions::tables::{lgdt, load_tss};
use x86_64::registers::model_specific::{GsBase, Star};
use x86_64::structures::gdt::{Descriptor, SegmentSelector};
use x86_64::structures::DescriptorTablePointer;
use x86_64::{PrivilegeLevel, VirtAddr};
```

#### 定义TSS类型
```
/// 如果编译时没有参数"ioport_bitmap", 则将TSS设置为x86_64结构下的TaskStateSegment
#[cfg(not(feature = "ioport_bitmap"))]
type TSS = x86_64::structures::tss::TaskStateSegment;

/// 如果编译时有参数"ioport_bitmap", 则将TSS设置为TSSWithPortBitmap
#[cfg(feature = "ioport_bitmap")]
type TSS = super::ioport::TSSWithPortBitmap;
```

#### 初始化TSS与GDT
```
pub fn init() {
}
```
在这个函数中，我们：
为trap分配栈，将栈顶设置为tss，这样一来，从ring3切换到ring0，CPU可以正确地切换栈
代码如下
```
/// 新建一个位于堆上的TSS结构，并创建一个位于栈中的指针tss指向它
/// https://docs.rs/x86_64/0.1.1/x86_64/structures/tss/struct.TaskStateSegment.html

let mut tss = Box::new(TSS::new());


/// 在堆中新建一个0x1000大小的数组，用于模拟栈，使用`Box::leak().as_ptr()`获取这个数组的起始地址，再加0x1000的偏移，就获得了这个模拟栈的栈顶

let trap_stack_top = Box::leak(Box::new([0u8; 0x1000])).as_ptr() as u64 + 0x1000;

/// 将栈顶设置为tss的privilege_stack_table
tss.privilege_stack_table[0] = VirtAddr::new(trap_stack_top);

/// 解开Box, 获取tss的数据 ***？
let tss: &'static _ = Box::leak(tss);

/// Creates a TSS system descriptor for the given TSS. 如果不能返回System Segment, 那么panic
/// tss0与tss1:
/// tss0: 按descriptor格式重写的栈顶地址
/// 如何理解tss1？
let (tss0, tss1) = match Descriptor::tss_segment(tss) {
        Descriptor::SystemSegment(tss0, tss1) => (tss0, tss1),
        _ => unreachable!(),
    };

// Extreme hack: the segment limit assumed by x86_64 does not include the port bitmap.
    #[cfg(feature = "ioport_bitmap")]
    let tss0 = (tss0 & !0xFFFF) | (size_of::<TSS>() as u64);

    unsafe {
        // get current GDT
        let gdtp = sgdt();
        let entry_count = (gdtp.limit + 1) as usize / size_of::<u64>();

    /// old_gdt: 一个slice, 从现有的gdt的base地址开始, 取entry_count个值
        let old_gdt = core::slice::from_raw_parts(gdtp.base as *const u64, entry_count);

        // allocate new GDT with 7 more entries
        //
        // NOTICE: for fast syscall:
        //   STAR[47:32] = K_CS   = K_SS - 8
        //   STAR[63:48] = U_CS32 = U_SS32 - 8 = U_CS - 16
        let mut gdt = Vec::from(old_gdt);
        gdt.extend([tss0, tss1, KCODE64, KDATA64, UCODE32, UDATA32, UCODE64].iter());
        let gdt = Vec::leak(gdt);


        // load new GDT and TSS
        lgdt(&DescriptorTablePointer {
            limit: gdt.len() as u16 * 8 - 1,
            base: gdt.as_ptr() as _,
        });

    /// SegmentSelector: a index to LDT or GDT table with some additional flags
    /// Load the task state register using the ltr instruction
        load_tss(SegmentSelector::new(
            entry_count as u16,
            PrivilegeLevel::Ring0,
        ));

        // for fast syscall:
        // store address of TSS to kernel_gsbase
        GsBase::MSR.write(tss as *const _ as u64);

        Star::write_raw(
            SegmentSelector::new(entry_count as u16 + 4, PrivilegeLevel::Ring3).0,
            SegmentSelector::new(entry_count as u16 + 2, PrivilegeLevel::Ring0).0,
        );
    }

```


#### 获取当前GDTR内容
```
/// Get current GDT register
#[inline]
unsafe fn sgdt() -> DescriptorTablePointer {
    let mut gdt = DescriptorTablePointer { limit: 0, base: 0 };
    asm!("sgdt [{}]", in(reg) &mut gdt);
    gdt
}
```


#### 定义了一些全局描述符
全局描述符表(GDT), It contains entries telling the CPU about memory segments. A similar Interrupts Descriptor Table exists containing tasks and interrupts descriptors.

![GDT_Entry](/docs/riscv_doc/GDT_Entry.png)
```
const KCODE64: u64 = 0x00209800_00000000; // EXECUTABLE | USER_SEGMENT | PRESENT | LONG_MODE
const UCODE64: u64 = 0x0020F800_00000000; // EXECUTABLE | USER_SEGMENT | USER_MODE | PRESENT | LONG_MODE
const KDATA64: u64 = 0x00009200_00000000; // DATA_WRITABLE | USER_SEGMENT | PRESENT
#[allow(dead_code)]
const UDATA64: u64 = 0x0000F200_00000000; // DATA_WRITABLE | USER_SEGMENT | USER_MODE | PRESENT
const UCODE32: u64 = 0x00cffa00_0000ffff; // EXECUTABLE | USER_SEGMENT | USER_MODE | PRESENT
const UDATA32: u64 = 0x00cff200_0000ffff; // EXECUTABLE | USER_SEGMENT | USER_MODE | PRESENT
```
以上述代码中的KCODE64为例分析:
<table border=0 cellpadding=0 cellspacing=0 width=576 style='border-collapse:
 collapse;table-layout:fixed;width:432pt'>
 <col width=64 span=9 style='width:48pt'>
 <tr height=19 style='height:14.4pt'>
  <td colspan=4 height=19 class=xl68 width=256 style='height:14.4pt;width:192pt'>Base
  0:15</td>
  <td colspan=4 class=xl69 width=256 style='border-left:none;width:192pt'>Limit
  0:15</td>
  <td width=64 style='width:48pt'></td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl66 style='height:14.4pt;border-top:none'>0000</td>
  <td class=xl66 style='border-top:none;border-left:none'>0000</td>
  <td class=xl66 style='border-top:none;border-left:none'>0000</td>
  <td class=xl66 style='border-top:none;border-left:none'>0000</td>
  <td class=xl66 style='border-top:none;border-left:none'>0000</td>
  <td class=xl66 style='border-top:none;border-left:none'>0000</td>
  <td class=xl66 style='border-top:none;border-left:none'>0000</td>
  <td class=xl66 style='border-top:none;border-left:none'>0000</td>
  <td></td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td colspan=2 height=19 class=xl70 style='height:14.4pt'>Base 24:31</td>
  <td class=xl66 style='border-top:none;border-left:none'>Flags</td>
  <td class=xl71 style='border-top:none;border-left:none'>Limit 16:19</td>
  <td colspan=2 class=xl72 style='border-left:none'>Access Byte</td>
  <td colspan=2 class=xl70 style='border-left:none'>Base 16:23</td>
  <td></td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl66 style='height:14.4pt;border-top:none'>0000</td>
  <td class=xl66 style='border-top:none;border-left:none'>0000</td>
  <td class=xl67 style='border-top:none;border-left:none'>0010</td>
  <td class=xl67 style='border-top:none;border-left:none'>0000</td>
  <td class=xl67 style='border-top:none;border-left:none'>1001</td>
  <td class=xl67 style='border-top:none;border-left:none'>1000</td>
  <td class=xl66 style='border-top:none;border-left:none'>0000</td>
  <td class=xl66 style='border-top:none;border-left:none'>0000</td>
  <td></td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 colspan=9 style='height:14.4pt;mso-ignore:colspan'></td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl75 style='height:14.4pt'>Base:</td>
  <td class=xl74>0</td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl73 style='height:14.4pt;border-top:none'>&nbsp;</td>
  <td class=xl100>&nbsp;</td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl101 style='height:14.4pt'>Limit:</td>
  <td class=xl76>0</td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl102 style='height:14.4pt;border-top:none'>&nbsp;</td>
  <td class=xl104>&nbsp;</td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
  <td class=xl65></td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td colspan=2 rowspan=2 height=38 class=xl79 style='border-right:.5pt solid black;
  border-bottom:.5pt solid black;height:28.8pt'>Access Byte:</td>
  <td class=xl82>Pr</td>
  <td class=xl77 style='border-left:none'>Privl</td>
  <td class=xl77 style='border-left:none'>S</td>
  <td class=xl77 style='border-left:none'>EX</td>
  <td class=xl77 style='border-left:none'>DC</td>
  <td class=xl77 style='border-left:none'>RW</td>
  <td class=xl77 style='border-left:none'>AC</td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl82 style='height:14.4pt;border-top:none'>1</td>
  <td class=xl78 style='border-top:none;border-left:none'>00</td>
  <td class=xl77 style='border-top:none;border-left:none'>1</td>
  <td class=xl77 style='border-top:none;border-left:none'>1</td>
  <td class=xl77 style='border-top:none;border-left:none'>0</td>
  <td class=xl77 style='border-top:none;border-left:none'>0</td>
  <td class=xl77 style='border-top:none;border-left:none'>0</td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl83 style='height:14.4pt'>&nbsp;</td>
  <td class=xl84>&nbsp;</td>
  <td colspan=7 class=xl89 style='border-bottom:.5pt solid black;border-left:
  none'>Present bit, must be 1</td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl85 style='height:14.4pt'>&nbsp;</td>
  <td class=xl65></td>
  <td class=xl90 style='border-top:none'>&nbsp;</td>
  <td colspan=6 class=xl91 style='border-bottom:.5pt solid black'>Privilege, 2
  bits,ring level. 00=highest(user applications)</td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl85 style='height:14.4pt'>&nbsp;</td>
  <td></td>
  <td class=xl92>&nbsp;</td>
  <td class=xl93 style='border-top:none'>&nbsp;</td>
  <td colspan=5 class=xl91 style='border-bottom:.5pt solid black'>Descripter
  Type. Set 1 for code or data segments</td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl85 style='height:14.4pt'>&nbsp;</td>
  <td></td>
  <td class=xl92>&nbsp;</td>
  <td class=xl94></td>
  <td class=xl93 style='border-top:none'>&nbsp;</td>
  <td colspan=4 class=xl91 style='border-bottom:.5pt solid black'>Executable
  bit. 1 for executable</td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl85 style='height:14.4pt'>&nbsp;</td>
  <td></td>
  <td class=xl92>&nbsp;</td>
  <td class=xl94></td>
  <td class=xl94></td>
  <td class=xl93 style='border-top:none'>&nbsp;</td>
  <td colspan=3 class=xl91 style='border-bottom:.5pt solid black'>Direction
  bit. 0 grows up.</td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl85 style='height:14.4pt'>&nbsp;</td>
  <td></td>
  <td class=xl92>&nbsp;</td>
  <td class=xl94></td>
  <td class=xl94></td>
  <td class=xl94></td>
  <td class=xl93 style='border-top:none'>&nbsp;</td>
  <td colspan=2 class=xl91 style='border-bottom:.5pt solid black'>Readable/Writable</td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl86 style='height:14.4pt'>&nbsp;</td>
  <td></td>
  <td class=xl95>&nbsp;</td>
  <td class=xl96>&nbsp;</td>
  <td class=xl96>&nbsp;</td>
  <td class=xl96>&nbsp;</td>
  <td class=xl96>&nbsp;</td>
  <td class=xl97 style='border-top:none'>&nbsp;</td>
  <td class=xl98 style='border-top:none'>Accessed bit.</td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl86 style='height:14.4pt'>&nbsp;</td>
  <td class=xl107>&nbsp;</td>
  <td class=xl96>&nbsp;</td>
  <td class=xl96>&nbsp;</td>
  <td class=xl96>&nbsp;</td>
  <td class=xl96>&nbsp;</td>
  <td class=xl94></td>
  <td class=xl94></td>
  <td class=xl105></td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td colspan=2 rowspan=2 height=38 class=xl99 style='height:28.8pt'>Flags:</td>
  <td class=xl88 style='border-top:none;border-left:none'>Gr</td>
  <td class=xl88 style='border-top:none;border-left:none'>Sz</td>
  <td class=xl88 style='border-top:none;border-left:none'>0</td>
  <td class=xl88 style='border-top:none;border-left:none'>0</td>
  <td colspan=3 style='mso-ignore:colspan'></td>
 </tr>
 <tr height=19 style='height:14.4pt'>
  <td height=19 class=xl99 style='height:14.4pt;border-top:none;border-left:
  none'>0</td>
  <td class=xl99 style='border-top:none;border-left:none'>0</td>
  <td class=xl99 style='border-top:none;border-left:none'>1</td>
  <td class=xl99 style='border-top:none;border-left:none'>0</td>
  <td colspan=3 style='mso-ignore:colspan'></td>
 </tr>
 <![if supportMisalignedColumns]>
 <tr height=0 style='display:none'>
  <td width=64 style='width:48pt'></td>
  <td width=64 style='width:48pt'></td>
  <td width=64 style='width:48pt'></td>
  <td width=64 style='width:48pt'></td>
  <td width=64 style='width:48pt'></td>
  <td width=64 style='width:48pt'></td>
  <td width=64 style='width:48pt'></td>
  <td width=64 style='width:48pt'></td>
  <td width=64 style='width:48pt'></td>
 </tr>
 <![endif]>
</table>