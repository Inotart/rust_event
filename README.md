# rust_event
Rust 轻量化异步事件库
> 作者: InotArt

这是一个基于tokio制作的一个轻量化异步事件库,安装方式：
> cargo add rust_event
## 使用方法:
```rust
event::event!(
    #[doc = "plugin message event"] 
    PluginMessageEvent(String, Vec<u8>)
);
pub async fn handle_plugin_message(player:String, data:Vec<u8>) {
    println!("[PluginMessage] {}: {:?}",player,data);
}
#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    // Here, "a" and "b" are just the names of the parameters. These names can be arbitrarily chosen and they correspond to the "player" and "data" of the function.
    event::event_register_global_async!(PluginMessageEvent, handle_plugin_message,(a,b));// register function
    event::event_emit_global!(PluginMessageEvent,("Earth Online".to_string(), vec![42]));// send data to function
    tokio::time::sleep(std::time::Duration::from_secs(1)).await; // Wait for one second to avoid exiting too early.
    Ok(())
}

```
```rust
event::event!(
    #[doc = "plugin message event"] 
    PluginMessageEvent(String, Vec<u8>)
);
pub async fn handle_plugin_message(player:String, data:Vec<u8>) {
    println!("[PluginMessage] {}: {:?}",player,data);
}
#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let bus = event::AsyncEventBus::new();
    // Here, "a" and "b" are just the names of the parameters. These names can be arbitrarily chosen and they correspond to the "player" and "data" of the function.
    event::event_register_async!(bus,PluginMessageEvent, handle_plugin_message,(a,b));// register function
    bus.emit::<PluginMessageEvent>(("Earth Online".to_string(), vec![42])).await;// send data to function
    // or
    event::event_emit!(bus,PluginMessageEvent,("Earth Online".to_string(), vec![42]));// send data to function
    tokio::time::sleep(std::time::Duration::from_secs(1)).await; // Wait for one second to avoid exiting too early.
    Ok(())
}

```
