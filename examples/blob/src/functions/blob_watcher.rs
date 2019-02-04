use azure_functions::{func, bindings::BlobTrigger};

#[func]
#[binding(name = "trigger", path = "watching/{name}")]
pub fn blob_watcher(trigger: &BlobTrigger) {
    log::info!(
        "A blob was created at '{}' with contents: {:?}.",
        trigger.path,
        trigger.blob
    );
}
