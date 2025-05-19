//! # Implementation of normalized line endings

/// Trait for iterating over characters with normalized line endings.
pub trait Normalized: Iterator<Item = char> {
  /// Returns an iterator over characters with all line endings converted to `\n`.
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

/// Returns an iterator over characters with all line endings converted to `\n`.
pub fn normalized(iter: impl Iterator<Item = char>) -> impl Iterator<Item = char> {
  NormalizedLineEndings { iter, state: State::AnyCharacter }
}

/// Line feed.
const LF: char = '\n';

/// Carriage return.
const CR: char = '\r';

enum State {
  AnyCharacter,
  CarriageReturn,
}

struct NormalizedLineEndings<I> {
  iter: I,
  state: State,
}

impl<I> Iterator for NormalizedLineEndings<I>
where
  I: Iterator<Item = char>,
{
  type Item = char;

  fn next(&mut self) -> Option<char> {
    match self.iter.next() {
      Some(LF) => match self.state {
        State::CarriageReturn => match self.iter.next() {
          Some(CR) => Some(LF),
          other => {
            self.state = State::AnyCharacter;
            other
          }
        },
        State::AnyCharacter => Some(LF),
      },
      Some(CR) => {
        self.state = State::CarriageReturn;
        Some(LF)
      }
      other => {
        self.state = State::AnyCharacter;
        other
      }
    }
  }
}
