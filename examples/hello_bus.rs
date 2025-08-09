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
