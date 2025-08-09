use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use futures::future::BoxFuture;
use futures::FutureExt;
use tokio::sync::RwLock;

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

#[macro_export]
macro_rules! event_emit {
    ($bus:expr, $event:ty, $data:expr) => {{
        $bus.emit::<$event>($data).await;
    }};
}
#[macro_export]
macro_rules! event_register_async {
    ($bus:expr, $event:ty, $handler:path, ($($param:ident),+)) => {{
        let bus_clone = $bus.clone();
        use futures::future::FutureExt; 
        bus_clone.register::<$event>(move |($($param),+)| {
            async move { $handler($($param),+).await }.boxed()
        }).await;
    }};
}
#[macro_export]
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
use once_cell::sync::Lazy;

pub static GLOBAL_EVENT_BUS: Lazy<Arc<AsyncEventBus>> = Lazy::new(|| {
    AsyncEventBus::new()
});

#[macro_export]
macro_rules! event_register_global_async {
    ($event:ty, $handler:path, ($($param:ident),+)) => {{
        use $crate::GLOBAL_EVENT_BUS;
        use futures::future::FutureExt; 
        let bus = GLOBAL_EVENT_BUS.clone();
        tokio::spawn(async move {
            bus.register::<$event>(move |($($param),+)| {
                async move { $handler($($param),+).await }.boxed()
            }).await;
        });
    }};
}
#[macro_export]
macro_rules! event_emit_global {
    ($event:ty, $data:expr) => {{
        use $crate::GLOBAL_EVENT_BUS;
        let bus = GLOBAL_EVENT_BUS.clone();
        tokio::spawn(async move {
            bus.emit::<$event>($data).await;
        });
    }};
}