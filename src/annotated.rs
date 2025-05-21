//! # Annotated iterator implementation

use crate::common::{CR, LF};
use crate::line_ending::LineEnding;

/// Enumeration of annotated characters.
///
/// The character can be any Unicode character (excluding line endings)
/// or a line ending itself. Additionally, each enumeration variant stores
/// the row and column position where the character or line ending
/// was encountered in the source text.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AnnotatedChar {
  /// Any character except line ending.
  ///
  /// This enumeration variant holds the character, row and column.
  Character(char, usize, usize),
  /// Line ending.
  ///
  /// This enumeration variant holds the line ending, row and column.
  LineEnding(LineEnding, usize, usize),
}

/// Trait extension for iterating over annotated characters.
pub trait Annotated: Iterator<Item = char> {
  /// Returns an iterator over annotated characters.
  ///
  /// # Examples
  ///
  /// ```
  /// use normalized_line_endings::{Annotated, AnnotatedChar, LineEnding};
  ///
  /// let mut chars = "Lorem\r\nipsum".chars().annotated();
  /// assert_eq!(AnnotatedChar::Character('L', 1, 1), chars.next().unwrap());
  /// assert_eq!(AnnotatedChar::Character('o', 1, 2), chars.next().unwrap());
  /// assert_eq!(AnnotatedChar::Character('r', 1, 3), chars.next().unwrap());
  /// assert_eq!(AnnotatedChar::Character('e', 1, 4), chars.next().unwrap());
  /// assert_eq!(AnnotatedChar::Character('m', 1, 5), chars.next().unwrap());
  /// assert_eq!(AnnotatedChar::LineEnding(LineEnding::CrLf, 1, 6), chars.next().unwrap());
  /// assert_eq!(AnnotatedChar::Character('i', 2, 1), chars.next().unwrap());
  /// assert_eq!(AnnotatedChar::Character('p', 2, 2), chars.next().unwrap());
  /// assert_eq!(AnnotatedChar::Character('s', 2, 3), chars.next().unwrap());
  /// assert_eq!(AnnotatedChar::Character('u', 2, 4), chars.next().unwrap());
  /// assert_eq!(AnnotatedChar::Character('m', 2, 5), chars.next().unwrap());
  /// ```
  fn annotated(self) -> impl Iterator<Item = AnnotatedChar>;
}

impl<I> Annotated for I
where
  I: Iterator<Item = char>,
{
  fn annotated(self) -> impl Iterator<Item = AnnotatedChar> {
    annotated(self)
  }
}

/// Returns an iterator over annotated characters.
///
/// # Examples
///
/// ```
/// use normalized_line_endings::{annotated, AnnotatedChar, LineEnding};
///
/// let mut chars = annotated("Lorem\r\nipsum".chars());
/// assert_eq!(AnnotatedChar::Character('L', 1, 1), chars.next().unwrap());
/// assert_eq!(AnnotatedChar::Character('o', 1, 2), chars.next().unwrap());
/// assert_eq!(AnnotatedChar::Character('r', 1, 3), chars.next().unwrap());
/// assert_eq!(AnnotatedChar::Character('e', 1, 4), chars.next().unwrap());
/// assert_eq!(AnnotatedChar::Character('m', 1, 5), chars.next().unwrap());
/// assert_eq!(AnnotatedChar::LineEnding(LineEnding::CrLf, 1, 6), chars.next().unwrap());
/// assert_eq!(AnnotatedChar::Character('i', 2, 1), chars.next().unwrap());
/// assert_eq!(AnnotatedChar::Character('p', 2, 2), chars.next().unwrap());
/// assert_eq!(AnnotatedChar::Character('s', 2, 3), chars.next().unwrap());
/// assert_eq!(AnnotatedChar::Character('u', 2, 4), chars.next().unwrap());
/// assert_eq!(AnnotatedChar::Character('m', 2, 5), chars.next().unwrap());
/// ```
pub fn annotated(iter: impl Iterator<Item = char>) -> impl Iterator<Item = AnnotatedChar> {
  AnnotatedCharacters {
    iter,
    peeked: None,
    row: 1,
    column: 0,
  }
}

struct AnnotatedCharacters<I> {
  iter: I,
  peeked: Option<char>,
  row: usize,
  column: usize,
}

impl<I: Iterator<Item = char>> Iterator for AnnotatedCharacters<I> {
  type Item = AnnotatedChar;

  fn next(&mut self) -> Option<AnnotatedChar> {
    let row = self.row;
    let column = self.column + 1;
    match self.peeked.take().or_else(|| self.iter.next()) {
      Some(LF) => {
        self.row += 1;
        self.column = 0;
        Some(AnnotatedChar::LineEnding(LineEnding::Lf, row, column))
      }
      Some(CR) => {
        self.row += 1;
        self.column = 0;
        if let Some(ch) = self.iter.next() {
          if ch == LF {
            self.peeked = None;
            Some(AnnotatedChar::LineEnding(LineEnding::CrLf, row, column))
          } else {
            self.peeked = Some(ch);
            Some(AnnotatedChar::LineEnding(LineEnding::Cr, row, column))
          }
        } else {
          self.peeked = None;
          Some(AnnotatedChar::LineEnding(LineEnding::Cr, row, column))
        }
      }
      other => {
        self.column = column;
        other.map(|ch| AnnotatedChar::Character(ch, row, column))
      }
    }
  }
}
