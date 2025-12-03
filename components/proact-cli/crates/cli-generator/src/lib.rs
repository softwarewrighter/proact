//! Documentation generator for proact

mod generate;
mod legal;

pub use generate::generate_documentation;
pub use legal::{copy_templates, generate_legal_files};
