#![no_std]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]

//! # Line endings normalizer
//!
//! ```
//! use normalized_line_endings::Normalized;
//!
//! let input = "Lorem \n ipsum \r dolor \n\r\n sit \r\r\n\n amet";
//! assert_eq!(
//!   "Lorem \n ipsum \n dolor \n\n sit \n\n\n amet",
//!   input.chars().normalized().collect::<String>()
//! );
//! ```

/// Trait extension for iterating over characters with normalized line endings.
pub trait Normalized: Iterator<Item = char> {
    /// Returns an iterator over characters with normalized line endings.
    ///
    /// # Examples
    ///
    /// ```
    /// use normalized_line_endings::Normalized;
    ///
    /// let input = "Lorem \n ipsum \r dolor \n\r\n sit \r\r\n\n amet";
    /// assert_eq!(
    ///   "Lorem \n ipsum \n dolor \n\n sit \n\n\n amet",
    ///   input.chars().normalized().collect::<String>()
    /// );
    /// ```
    fn normalized(self) -> impl Iterator<Item = char>
    where
        Self: Sized,
    {
        Normalizer {
            iter: self,
            next: None,
        }
    }
}

/// Line endings normalizer.
struct Normalizer<I: Iterator<Item = char>> {
    iter: I,
    next: Option<char>,
}

impl<I: Iterator<Item = char>> Iterator for Normalizer<I> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.next.take().or_else(|| self.iter.next()) {
            Some('\r') => {
                self.next = self.iter.next().filter(|ch| *ch != '\n');
                Some('\n')
            }
            other => other,
        }
    }
}

/// Implement normalizer for iterator over characters.
impl<I: Iterator<Item = char>> Normalized for I {}
