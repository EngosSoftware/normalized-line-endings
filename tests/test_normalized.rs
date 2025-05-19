use peekable_normalized_line_endings::{normalized, Normalized};
use std::iter::FromIterator;

#[test]
fn _0001() {
  let input = "Lorem \n ipsum \r dolor \r\n sit \n\r\n amet, consectetur \r\r\n\n adipiscing \n\n\r\r elit \r\n\r.";
  assert_eq!(
    "Lorem \n ipsum \n dolor \n sit \n\n amet, consectetur \n\n\n adipiscing \n\n\n\n elit \n\n.",
    &String::from_iter(normalized(input.chars()))
  );
}

#[test]
fn _0002() {
  let input = "Lorem \n ipsum \r dolor \r\n sit \n\r\n amet, consectetur \r\r\n\n adipiscing \n\n\r\r elit \r\n\r.";
  assert_eq!(
    "Lorem \n ipsum \n dolor \n sit \n\n amet, consectetur \n\n\n adipiscing \n\n\n\n elit \n\n.",
    input.chars().normalized().collect::<String>(),
  );
}
