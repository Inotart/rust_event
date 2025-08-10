# rust_event
Rust 轻量化异步事件库
> 作者: InotArt

这是一个基于tokio制作的一个轻量化异步事件库,安装方式：
> cargo add rust_event
## 使用方法:
### 新版本
#### 简易使用
```rust
rust_event::event!(
    #[doc = "plugin message event"] 
    PluginMessageEvent(String, Vec<u8>)
);
// Here, "a" and "b" are just the names of the parameters. These names can be arbitrarily chosen and they correspond to the "player" and "data" of the function.
rust_event::event_global_async!(PluginMessageEvent, handle_plugin_message,(a,b));
pub async fn handle_plugin_message(player:String, data:Vec<u8>) {
    println!("[PluginMessage] {}: {:?}",player,data);
}
#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await; // Wait for one second to await rust_event::event_global_async
    rust_event::event_emit_global!(PluginMessageEvent,("Earth Online".to_string(), vec![42]));// send data to function
    tokio::time::sleep(std::time::Duration::from_secs(1)).await; // Wait for one second to avoid exiting too early.
    Ok(())
}

```
#### 自定义全局总线
```rust
rust_event::event_summon_global_bus!(BUS);
rust_event::event!(
    #[doc = "插件消息事件"] 
    PluginMessageEvent(String, String)
);
rust_event::event_async!(BUS, PluginMessageEvent, handle_plugin_message,(a,b));
pub async fn handle_plugin_message(player:String, text:String) {
    println!("[PluginMessage] {}: {}",player, text);
}
#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;// 注册是异步的,所以需要等会
    BUS.emit::<PluginMessageEvent>(("Alice".to_string(), "Hello Event!".to_string())).await;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(())
}

```
### 旧版本(非解耦版本)
#### 简易使用
```rust
rust_event::event!(
    #[doc = "plugin message event"] 
    PluginMessageEvent(String, Vec<u8>)
);
pub async fn handle_plugin_message(player:String, data:Vec<u8>) {
    println!("[PluginMessage] {}: {:?}",player,data);
}
#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    // Here, "a" and "b" are just the names of the parameters. These names can be arbitrarily chosen and they correspond to the "player" and "data" of the function.
    rust_event::event_register_global_async!(PluginMessageEvent, handle_plugin_message,(a,b));// register function
    rust_event::event_emit_global!(PluginMessageEvent,("Earth Online".to_string(), vec![42]));// send data to function
    tokio::time::sleep(std::time::Duration::from_secs(1)).await; // Wait for one second to avoid exiting too early.
    Ok(())
}
```
#### 自定义全局总线
```rust
rust_event::event!(
    #[doc = "plugin message event"] 
    PluginMessageEvent(String, Vec<u8>)
);
pub async fn handle_plugin_message(player:String, data:Vec<u8>) {
    println!("[PluginMessage] {}: {:?}",player,data);
}
#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let bus = rust_event::AsyncEventBus::new();
    // Here, "a" and "b" are just the names of the parameters. These names can be arbitrarily chosen and they correspond to the "player" and "data" of the function.
    rust_event::event_register_async!(bus,PluginMessageEvent, handle_plugin_message,(a,b));// register function
    bus.emit::<PluginMessageEvent>(("Earth Online".to_string(), vec![42])).await;// send data to function
    // or
    rust_event::event_emit!(bus,PluginMessageEvent,("Earth Online".to_string(), vec![42]));// send data to function
    tokio::time::sleep(std::time::Duration::from_secs(1)).await; // Wait for one second to avoid exiting too early.
    Ok(())
}

```
