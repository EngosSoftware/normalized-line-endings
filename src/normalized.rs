/// Line feed.
const LF: char = '\n';

/// Carriage return.
const CR: char = '\r';

struct NormalizedLineEndings<I> {
  iter: I,
  prev_was_cr: bool,
}

enum State {
  AnyCharacter,
  CarriageReturn,
}

pub fn normalized(iter: impl Iterator<Item = char>) -> impl Iterator<Item = char> {
  NormalizedLineEndings { iter, prev_was_cr: false }
}

impl<I> Iterator for NormalizedLineEndings<I>
where
  I: Iterator<Item = char>,
{
  type Item = char;

  fn next(&mut self) -> Option<char> {
    match self.iter.next() {
      Some(LF) if self.prev_was_cr => {
        self.prev_was_cr = false;
        match self.iter.next() {
          Some(CR) => {
            self.prev_was_cr = true;
            Some(LF)
          }
          any => {
            self.prev_was_cr = false;
            any
          }
        }
      }
      Some(CR) => {
        self.prev_was_cr = true;
        Some(LF)
      }
      any => {
        self.prev_was_cr = false;
        any
      }
    }
  }
}
