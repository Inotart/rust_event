use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use futures::future::BoxFuture;
use futures::FutureExt;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;
pub use futures;
pub use once_cell;
pub use paste;
pub use tokio;
pub use ctor;
trait AsyncEventHandler: Send + Sync {
    fn handle(&self, data: Box<dyn Any + Send>) -> BoxFuture<'static, ()>;
}

impl<F, Fut> AsyncEventHandler for F
where
    F: Fn(Box<dyn Any + Send>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    fn handle(&self, data: Box<dyn Any + Send>) -> BoxFuture<'static, ()> {
        (self)(data).boxed()
    }
}

// 事件总线结构体
#[derive(Default)]
pub struct AsyncEventBus {
    handlers: Arc<RwLock<HashMap<TypeId, Vec<Box<dyn AsyncEventHandler>>>>>,
}

impl AsyncEventBus {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn register<E: Event + 'static>(
        &self,
        handler: impl Fn(E::Data) -> BoxFuture<'static, ()> + 'static + Send + Sync,
    ) {
        let type_id = TypeId::of::<E>();
        let shared_handler = Arc::new(handler);

        let async_wrapper = move |data: Box<dyn Any + Send>| {
            let inner_handler = Arc::clone(&shared_handler);
            async move {
                if let Ok(concrete) = data.downcast::<E::Data>() {
                    inner_handler(*concrete).await;
                }
            }
        };

        let mut handlers = self.handlers.write().await;
        handlers
            .entry(type_id)
            .or_default()
            .push(Box::new(async_wrapper));
    }

    pub async fn emit<E: Event + 'static>(&self, data: E::Data) {
        let handlers = self.handlers.read().await;
        if let Some(handlers) = handlers.get(&TypeId::of::<E>()) {
            for handler in handlers {
                // 关键修改：直接传递数据，不需要克隆
                handler.handle(Box::new(data.clone())).await;
            }
        }
    }
    pub async fn has_handlers<E: Event + 'static>(&self) -> bool {
        let handlers = self.handlers.read().await;
        handlers.contains_key(&TypeId::of::<E>())
    }
}
pub trait Event {
    type Data: Clone + Send + Sync + 'static;
}
pub static GLOBAL_EVENT_BUS: Lazy<Arc<AsyncEventBus>> = Lazy::new(|| {
    AsyncEventBus::new()
});
#[macro_export]
/// 向指定消息总线的事件发送消息
macro_rules! event_emit {
    ($bus:expr, $event:ty, $data:expr) => {{
        $bus.emit::<$event>($data).await;
    }};
}
#[macro_export]
/// 将函数异步注册到指定全局消息总线中的指定事件
macro_rules! event_register_async {
    ($bus:expr, $event:ty, $handler:path, ($($param:ident),+)) => {{
        let bus_clone = $bus.clone();
        use $crate::futures::future::FutureExt; 
        bus_clone.register::<$event>(move |($($param),+)| {
            async move { $handler($($param),+).await }.boxed()
        }).await;
    }};
}
#[macro_export]
/// 创建消息事件
macro_rules! event {
    ($name:ident($($field:ty),+)) => {
        pub struct $name;
        impl $crate::Event for $name {
            type Data = ($($field),+);
        }
    };
    (#[doc = $doc:expr] $name:ident($($field:ty),+)) => {
        #[doc = $doc]
        pub struct $name;
        impl $crate::Event for $name {
            type Data = ($($field),+);
        }
    };
}




#[macro_export]
/// 将函数异步注册到Rust_Event 库自带的全局消息总线中的指定事件
macro_rules! event_register_global_async {
    ($event:ty, $handler:path, ($($param:ident),+)) => {{
        use $crate::GLOBAL_EVENT_BUS;
        use $crate::futures::future::FutureExt; 
        let bus = GLOBAL_EVENT_BUS.clone();
        $crate::tokio::spawn(async move {
            bus.register::<$event>(move |($($param),+)| {
                async move { $handler($($param),+).await }.boxed()
            }).await;
        });
    }};
}
#[macro_export]
/// 向 Rust_Event 库自带的全局消息总线中指定事件类型发送消息
macro_rules! event_emit_global {
    ($event:ty, $data:expr) => {{
        use $crate::GLOBAL_EVENT_BUS;
        let bus = GLOBAL_EVENT_BUS.clone();
        $crate::tokio::spawn(async move {
            bus.emit::<$event>($data).await;
        });
    }};
}
#[macro_export]
/// 将消息注册到 Rust_Event 库自带的全局消息总线中的指定事件类型
macro_rules! event_global_async {
    ($event:ty, $handler:path, ($($param:ident),+)) => {
        paste::paste! {
            #[allow(non_snake_case)]
            fn [<__register_handler_ $event _ $handler>]() {
                use $crate::GLOBAL_EVENT_BUS;
                use $crate::futures::future::FutureExt;
                
                let bus = GLOBAL_EVENT_BUS.clone();
                
                // 正确方式：创建异步任务并 spawn
                let task = async move {
                    bus.register::<$event>(move |($($param),+)| {
                        async move { $handler($($param),+).await }.boxed()
                    }).await;
                };
                
                // 获取当前运行时或创建新运行时
                if let Ok(rt) = $crate::tokio::runtime::Handle::try_current() {
                    rt.spawn(task);
                } else {
                    ::std::thread::spawn(move || {
                        let rt = $crate::tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(task);
                    });
                }
            }
            
            // 使用 ctor 在程序启动时自动注册
            #[cfg_attr(not(test), ::ctor::ctor)]
            #[allow(non_snake_case)]
            fn [<__init_register_ $event _ $handler>]() {
                [<__register_handler_ $event _ $handler>]();
            }
            
            // 测试环境支持
            #[cfg(test)]
            #[::ctor::ctor]
            #[allow(non_snake_case)]
            fn [<__test_init_ $event _ $handler>]() {
                if std::thread::panicking() {
                    return;
                }
                [<__register_handler_ $event _ $handler>]();
            }
        }
    };
}
#[macro_export]
/// 将函数异步注册到指定消息总线中的指定事件
macro_rules! event_async {
    ($bus:expr, $event:ty, $handler:path, ($($param:ident),+)) => {
        paste::paste! {
            fn [<__register_handler_ $event:snake _ $handler:snake>]() {
                
                use $crate::futures::future::FutureExt;
                
                let [<bus_instance_ $event:snake _ $handler:snake>] = $bus.clone();
                
                let task = async move {
                    [<bus_instance_ $event:snake _ $handler:snake>].register::<$event>(move |($($param),+)| {
                        async move { $handler($($param),+).await }.boxed()
                    }).await;
                };
                
                if let Ok(rt) = $crate::tokio::runtime::Handle::try_current() {
                    rt.spawn(task);
                } else {
                    ::std::thread::spawn(move || {
                        let rt = $crate::tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(task);
                    });
                }
            }
            #[cfg_attr(not(test), ::ctor::ctor)]
            fn [<__init_register_ $event:snake _ $handler:snake>]() {
                [<__register_handler_ $event:snake _ $handler:snake>]();
            }

            #[cfg(test)]
            #[::ctor::ctor]
            fn [<__test_init_ $event:snake _ $handler:snake>]() {
                if ::std::thread::panicking() {
                    return;
                }
                [<__register_handler_ $event:snake _ $handler:snake>]();
            }
        }
    };
}
#[macro_export]
/// 创建一个全局消息总线
macro_rules! event_summon_global_bus {
    ($bus_name:ident) => {
        pub static $bus_name: $crate::once_cell::sync::Lazy<::std::sync::Arc<$crate::AsyncEventBus>> = 
            $crate::once_cell::sync::Lazy::new(|| $crate::AsyncEventBus::new());
    };
}