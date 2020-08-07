use super::*;
use spin::Mutex;
/// 空对象
#[derive(Debug)]
pub struct DummyObject {
    pub id: KoID,
    pub inner: Mutex<DummyObjectInner>,
}

/// DummyObject 的内部可变部分
/// 采用一种内部可变性的设计模式:
/// 将对象的所有可变的部分封装到一个内部对象DummyObjectInner中,
/// 并在原对象中用自旋锁Mutex把它包起来,
/// 因此此结构自动具有了Send+Sync的特性.
#[derive(Default, Debug)]
pub struct DummyObjectInner {
    pub name: String,
}
