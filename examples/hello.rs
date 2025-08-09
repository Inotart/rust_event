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
