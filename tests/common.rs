use normalized_line_endings::{CR, LF};

#[test]
fn _0001() {
  assert_eq!('\u{000A}', LF);
}

#[test]
fn _0002() {
  assert_eq!('\u{000D}', CR);
}
