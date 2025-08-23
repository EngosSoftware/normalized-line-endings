//! # Iterator over characters with normalized line endings
//!
//! The **Normalized line endings** crate provides an iterator over characters
//! with normalized line endings, meaning all valid line endings in the input
//! are converted to a single newline character `\n` (U+000A), like this:
//!
//! - `\n` → `\n`
//! - `\r` → `\n`
//! - `\r\n` → `\n`
//!
//! The normalized iterator can be created using standalone function [normalized()]
//! or by calling the method [Normalized::normalized] on any iterator over characters.
//!
//! # Examples
//!
//! ## Using standalone function [normalized()]
//!
//! ```
//! use normalized_line_endings::normalized;
//!
//! let input = "This is a string \n with \r some \n\r\n random newlines\r\r\n\n";
//! assert_eq!(
//!   "This is a string \n with \n some \n\n random newlines\n\n\n",
//!   normalized(input.chars()).collect::<String>()
//! );
//! ```
//!
//! ## Using [Normalized] trait extension
//!
//! ```
//! use normalized_line_endings::Normalized;
//!
//! let input = "This is a string \n with \r some \n\r\n random newlines\r\r\n\n";
//! assert_eq!(
//!   "This is a string \n with \n some \n\n random newlines\n\n\n",
//!   input.chars().normalized().collect::<String>()
//! );
//! ```

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
