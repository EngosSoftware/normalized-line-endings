//! # Iterator implementation

/// Trait extension for iterating over characters with normalized line endings.
pub trait Normalized: Iterator<Item = char> {
  /// Returns an iterator over characters with normalized line endings.
  ///
  /// # Examples
  ///
  /// ```
  /// use normalized_line_endings::Normalized;
  ///
  /// let input = "This is a string \n with \r some \n\r\n random newlines\r\r\n\n";
  /// assert_eq!(
  ///   "This is a string \n with \n some \n\n random newlines\n\n\n",
  ///   input.chars().normalized().collect::<String>()
  /// );
  /// ```
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
///
/// # Examples
///
/// ```
/// use normalized_line_endings::normalized;
///
/// let input = "This is a string \n with \r some \n\r\n random newlines\r\r\n\n";
/// assert_eq!(
///   "This is a string \n with \n some \n\n random newlines\n\n\n",
///   normalized(input.chars()).collect::<String>()
/// );
/// ```
pub fn normalized(iter: impl Iterator<Item = char>) -> impl Iterator<Item = char> {
  NormalizedLineEndings { iter, peeked: None }
}

struct NormalizedLineEndings<I> {
  iter: I,
  peeked: Option<char>,
}

impl<I: Iterator<Item = char>> Iterator for NormalizedLineEndings<I> {
  type Item = char;

  fn next(&mut self) -> Option<char> {
    match self.peeked.take().or_else(|| self.iter.next()) {
      some_lf @ Some('\n') => some_lf,
      Some('\r') => {
        self.peeked = self.iter.next().filter(|ch| *ch != '\n');
        Some('\n')
      }
      other => other,
    }
  }
}
