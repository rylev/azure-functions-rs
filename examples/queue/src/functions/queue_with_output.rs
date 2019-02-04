use azure_functions::{func, bindings::{QueueMessage, QueueTrigger}};

#[func]
#[binding(name = "trigger", queue_name = "echo-in")]
#[binding(name = "$return", queue_name = "echo-out")]
pub fn queue_with_output(trigger: &QueueTrigger) -> QueueMessage {
    log::info!("Message: {}", trigger.message);

    trigger.message.clone()
}
