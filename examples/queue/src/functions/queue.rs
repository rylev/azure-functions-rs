use azure_functions::{bindings::QueueTrigger, func};

#[func]
#[binding(name = "trigger", queue_name = "test")]
pub fn queue(trigger: &QueueTrigger) {
    info!("Message: {}", trigger.message);
}
