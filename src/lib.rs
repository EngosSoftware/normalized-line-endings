//! # Iterator over characters with normalized line endings

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]

mod normalized;

pub use normalized::{normalized, Normalized};
