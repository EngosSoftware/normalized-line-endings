//! # Normalized iterator implementation

/// Trait for iterating over characters with normalized line endings.
pub trait Normalized: Iterator<Item = char> {
  /// Returns an iterator over characters with normalized line endings.
  fn normalized(self) -> impl Iterator<Item = char>;
}

impl<I> Normalized for I
where
  I: Iterator<Item = char>,
{
  fn normalized(self) -> impl Iterator<Item = char> {
    normalized(self)
  }
}

/// Returns an iterator over characters with normalized line endings.
pub fn normalized(iter: impl Iterator<Item = char>) -> impl Iterator<Item = char> {
  NormalizedLineEndings { iter, peeked: None }
}

const CR: char = '\r';

const LF: char = '\n';

struct NormalizedLineEndings<I> {
  iter: I,
  peeked: Option<char>,
}

impl<I> Iterator for NormalizedLineEndings<I>
where
  I: Iterator<Item = char>,
{
  type Item = char;

  fn next(&mut self) -> Option<char> {
    match self.peeked.take().or_else(|| self.iter.next()) {
      Some(LF) => Some(LF),
      Some(CR) => {
        self.peeked = self.iter.next().filter(|ch| *ch != LF);
        Some(LF)
      }
      other => other,
    }
  }
}
