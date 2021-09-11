pub use defer::defer;
pub use error::{Ignore, SafeUnwrap};
pub use input::{input, input_string, prompt, Prompt};

mod defer;
mod error;
mod input;
