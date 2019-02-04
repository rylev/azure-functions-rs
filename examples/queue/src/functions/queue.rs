use azure_functions::{func, bindings::QueueTrigger};

#[func]
#[binding(name = "trigger", queue_name = "test")]
pub fn queue(trigger: &QueueTrigger) {
    info!("Message: {}", trigger.message);
}
