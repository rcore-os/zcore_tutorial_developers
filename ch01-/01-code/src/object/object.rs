use super::dummy_object::DummyObject;
use super::*;
use alloc::sync::Arc;
impl DummyObject {
    /// 创建一个新 DummyObject
    pub fn new() -> Arc<Self> {
        Arc::new(DummyObject {
            id: Self::new_koid(),
            inner: Default::default(),
        })
    }

    /// 生成一个唯一的ID
    /// 每个内核对象都有唯一的ID, 因此需要实现一个全局的ID分配方法。
    /// 这里用一个静态变量存放下一个待分配ID值，每次分配就原子地+1。
    /// 在Zircon中ID从1024开始分配，1024以下保留作内核内部使用
    /// ！！！ 注意：
    /// 这里new函数返回类型是Arc<Self>
    fn new_koid() -> KoID {
        static NEXT_KOID: AtomicU64 = AtomicU64::new(1024);
        NEXT_KOID.fetch_add(1, Ordering::SeqCst)
    }
}

impl KernelObject for DummyObject {
    fn id(&self) -> KoID {
        self.id
    }
    fn type_name(&self) -> &str {
        "DummyObject"
    }
    fn name(&self) -> String {
        self.inner.lock().name.clone()
    }
    fn set_name(&self, name: &str) {
        self.inner.lock().name = String::from(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dummy_object() {
        let o1 = DummyObject::new();
        let o2 = DummyObject::new();
        assert_ne!(o1.id(), o2.id());
        assert_eq!(o1.type_name(), "DummyObject");
        assert_eq!(o1.name(), "");
        o1.set_name("object1");
        assert_eq!(o1.name(), "object1");
    }
}
