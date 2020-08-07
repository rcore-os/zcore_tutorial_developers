use alloc::string::String;
use alloc::sync::Arc;
use core::fmt::Debug;
use core::sync::atomic::*;
use downcast_rs::{impl_downcast, DowncastSync};
use spin::*;
pub mod dummy_object;
pub mod object;
/// 内核对象公共接口
/// 这里的 Send + Sync 是一个约束所有内核对象都要满足的前提条件, 即它必须是一个并发对象，可以安全地被多线程访问

pub trait KernelObject: DowncastSync + Debug {
    /// 获取对象ID
    fn id(&self) -> KoID;
    /// 获取对象类型名
    fn type_name(&self) -> &str;
    /// 获取对象名称
    fn name(&self) -> String;
    /// 设置对象名称
    fn set_name(&self, name: &str);
}
impl_downcast!(sync KernelObject);
/// 内核对象核心结构
pub struct KObjectBase {
    /// 对象ID
    pub id: KoID,
    inner: Mutex<KObjectBaseInner>,
}

/// KObjectBase 的内部可变部分
#[derive(Default)]
pub struct KObjectBaseInner {
    name: String,
}

impl Default for KObjectBase {
    fn default() -> Self {
        KObjectBase {
            id: Self::new_koid(),
            inner: Default::default(),
        }
    }
}

impl KObjectBase {
    fn new_koid() -> KoID {
        static NEXT_KOID: AtomicU64 = AtomicU64::new(1024);
        NEXT_KOID.fetch_add(1, Ordering::SeqCst)
    }
    pub fn name(&self) -> String {
        self.inner.lock().name.clone()
    }
    pub fn set_name(&self, name: &str) {
        self.inner.lock().name = String::from(name);
    }
}
/// 对象ID 类型
pub type KoID = u64;

#[macro_export]
macro_rules! impl_kobject {
    ($class:ident $( $fn:tt)*) => {
        impl KernelObject for $class {
            fn id(&self) -> KoID {
                self.base.id
            }
            fn type_name(&self) -> &str {
                stringify!($class)
            }
            fn name(&self) -> alloc::string::String {
                self.base.name()
            }
            fn set_name(&self, name: &str){
                self.base.set_name(name)
            }
            $( $fn)*
        }
        impl core::fmt::Debug for $class {
            fn fmt(&self,
                f: &mut core::fmt::Formatter<'_>
            ) -> core::result::Result<(), core::fmt::Error> {
                f.debug_tuple(&stringify!($class))
                    .field(&self.id())
                    .field(&self.name())
                    .finish()
            }
        }
    };
}

pub struct DummyObjectPlus {
    base: KObjectBase,
}

impl_kobject!(DummyObjectPlus);

impl DummyObjectPlus {
    pub fn new() -> Arc<Self> {
        Arc::new(DummyObjectPlus {
            base: KObjectBase::default(),
        })
    }
}
#[test]
fn downcast() {
    let dummy = dummy_object::DummyObject::new();
    let object: Arc<dyn KernelObject> = dummy;
    let _result: Arc<dummy_object::DummyObject> =
        object.downcast_arc::<dummy_object::DummyObject>().unwrap();
}
#[test]
fn impl_kobject() {
    use alloc::format;
    let dummy = DummyObjectPlus::new();
    let object: Arc<dyn KernelObject> = dummy;
    assert_eq!(object.type_name(), "DummyObjectPlus");
    assert_eq!(object.name(), "");
    object.set_name("dummy");
    assert_eq!(object.name(), "dummy");
    assert_eq!(
        format!("{:?}", object),
        format!("DummyObjectPlus({}, \"dummy\")", object.id())
    );
    let _result = object.downcast_arc::<DummyObjectPlus>().unwrap();
}
