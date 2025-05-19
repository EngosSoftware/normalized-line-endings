use normalized_line_endings::{normalized, Normalized};
use std::iter::Peekable;
use std::str::Chars;

const INPUT: &str = "Lorem \n ipsum \r dolor \r\n sit \n\r\n amet, consectetur \r\r\n\n adipiscing \n\n\r\r elit \r\n\r.";
const OUTPUT: &str = "Lorem \n ipsum \n dolor \n sit \n\n amet, consectetur \n\n\n adipiscing \n\n\n\n elit \n\n.";

#[test]
fn _0001() {
  assert_eq!(OUTPUT, normalized(INPUT.chars()).collect::<String>());
}

#[test]
fn _0002() {
  assert_eq!(OUTPUT, INPUT.chars().normalized().collect::<String>());
}

#[test]
fn _0003() {
  eq(&mut OUTPUT.chars(), &mut normalized(INPUT.chars()).peekable());
}

#[test]
fn _0004() {
  eq(&mut OUTPUT.chars(), &mut INPUT.chars().normalized().peekable());
}

#[test]
fn _0005() {
  struct Tokenizer<I: Iterator<Item = char>> {
    chars: Peekable<I>
  }
  let mut tokenizer = Tokenizer{chars: INPUT.chars().normalized().peekable()};
  eq(&mut OUTPUT.chars(), &mut tokenizer.chars);
}

fn eq(expected_chars: &mut Chars, peekable_chars: &mut Peekable<impl Iterator<Item = char>>) {
  assert_eq!(expected_chars.next().unwrap(), peekable_chars.next().unwrap());
  while peekable_chars.peek().is_some() {
    let ch = expected_chars.next().unwrap();
    assert_eq!(ch, *peekable_chars.peek().unwrap());
    assert_eq!(ch, peekable_chars.next().unwrap());
  }
  assert!(peekable_chars.next().is_none());
  assert!(expected_chars.next().is_none());
}
