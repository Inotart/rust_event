use event::{event, event_register_async, AsyncEventBus};

// const bus = Event::AsyncEventBus
event!(
    #[doc = "玩家聊天消息事件"] 
    PluginMessageEvent(String, String)
);

pub async fn handle_plugin_message(player:String, text:String) {
    println!("[PluginMessage] {}: {}",player, text);
}
/// 消息事件
// event!(PluginMessageEvent(String, String))
// event!(PluginMessageEvent(String, String));
// event_handle!(bus)
#[tokio::test]
pub async fn no_global() -> Result<(), anyhow::Error> {
    let bus = AsyncEventBus::new();
    event_register_async!(bus, PluginMessageEvent, handle_plugin_message,(a,b));
    event_register_async!(bus, PluginMessageEvent, handle_plugin_message,(a,b));
    bus.emit::<PluginMessageEvent>(("Alice".to_string(), "Hello Event!".to_string())).await;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    Ok(())
}
