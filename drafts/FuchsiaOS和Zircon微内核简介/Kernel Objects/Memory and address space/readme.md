
## VMAR
- 类似于rCore中memory_set
- 树状结构
- inner包含
    - children, VMAR的vector
    - mapping, VMO的映射关系
- 成员函数存在递归情况（树的遍历）
- 也是一个kernel object
    - handle
    - rights
- 映射的对象只可能是VMO（参数传进来的只有VMO）
- 成员函数大概可以分为两类
    - export function （pub）
        - 供操作系统内部使用
        - 经过zircon-syscall包装之后变为syscall给用户程序使用
    - auxiliary function （within，contains……）
