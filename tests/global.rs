use event::{event_emit_global, event_register_global_async, Event};


// 定义事件
pub struct GlobalMessageEvent;
impl Event for GlobalMessageEvent {
    type Data = (String, String);
}

pub struct GlobalCounterEvent;
impl Event for GlobalCounterEvent {
    type Data = u32;
}

pub struct SystemAlertEvent;
impl Event for SystemAlertEvent {
    type Data = String;
}

// 事件处理器
pub async fn handle_global_message(sender: String, message: String) {
    println!("[Global Message] From {}: {}", sender, message);
}

pub async fn handle_global_counter(count: u32) {
    println!("[Global Counter] Current count: {}", count);
    if count % 10 == 0 {
        println!("[Global Counter] Milestone: Reached {}!", count);
    }
}

pub async fn handle_system_alert(message: String) {
    println!("[SYSTEM ALERT] {}", message);
}

// 测试函数
#[tokio::test]
pub async fn test_global_bus() -> Result<(), anyhow::Error> {
    println!("Starting global event bus test...");
    
    // 注册事件处理器
    event_register_global_async!(GlobalMessageEvent, handle_global_message, (sender, message));
    event_register_global_async!(GlobalCounterEvent, handle_global_counter, (count));
    event_register_global_async!(SystemAlertEvent, handle_system_alert, (message));
    
    println!("Registered global event handlers");
    
    // 等待注册完成
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // 发送消息事件
    event_emit_global!(
        GlobalMessageEvent, 
        ("Global Alice".to_string(), "Hello from global!".to_string())
    );
    
    event_emit_global!(
        GlobalMessageEvent, 
        ("Global Bob".to_string(), "This is a global message".to_string())
    );
    
    // 发送计数器事件
    for i in 1..=20 {
        event_emit_global!(GlobalCounterEvent, i);
        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
    }
    
    // 发送系统警报
    event_emit_global!(
        SystemAlertEvent, 
        "Critical system update required!".to_string()
    );
    
    // 等待事件处理完成
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    println!("Global event bus test completed");
    Ok(())
}