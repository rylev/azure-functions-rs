mod queue;
mod queue_with_output;

use azure_functions::{export, codegen::Function};

pub const FUNCTIONS: &[&Function] = export! {
    queue::queue,
    queue_with_output::queue_with_output,
};
