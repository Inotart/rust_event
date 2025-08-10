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
