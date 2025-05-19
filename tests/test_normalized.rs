use std::iter::FromIterator;
use peekable_normalized_line_endings::normalized;

#[test]
fn test_normalized() {
  let input = "Lorem \n ipsum \r dolor \r\n sit \n\r\n amet, consectetur \r\r\n\n adipiscing elit.";
  assert_eq!(
    &String::from_iter(normalized(input.chars())),
    "Lorem \n ipsum \n dolor \n sit \n\n amet, consectetur \n\n\n adipiscing elit."
  );
}