#![doc = include_str!("../readme/README_BODY.md")]

#![no_std]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]

mod annotated;
mod common;
mod line_ending;
mod normalized;

pub use annotated::{annotated, Annotated, AnnotatedChar};
pub use common::{CR, LF};
pub use line_ending::LineEnding;
pub use normalized::{normalized, Normalized};
