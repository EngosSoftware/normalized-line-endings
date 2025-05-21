use normalized_line_endings::{annotated, Annotated, AnnotatedChar, LineEnding};

fn line_ending_to_string(line_ending: LineEnding) -> String {
  match line_ending {
    LineEnding::Lf => "LF",
    LineEnding::Cr => "CR",
    LineEnding::CrLf => "CRLF",
  }
  .to_string()
}

fn annotated_char_to_string(annotated_char: AnnotatedChar) -> String {
  match annotated_char {
    AnnotatedChar::Character(ch, row, col) => format!("{:?}:{}:{}", ch, row, col),
    AnnotatedChar::LineEnding(line_ending, row, col) => format!("{}:{}:{}", line_ending_to_string(line_ending), row, col),
  }
}

fn annotated_to_string(input: &str) -> String {
  input.chars().annotated().map(annotated_char_to_string).collect::<Vec<String>>().join(", ")
}

#[test]
fn _0001() {
  let mut chars = "Lorem\r\nipsum".chars().annotated();
  assert_eq!(AnnotatedChar::Character('L', 1, 1), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('o', 1, 2), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('r', 1, 3), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('e', 1, 4), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('m', 1, 5), chars.next().unwrap());
  assert_eq!(AnnotatedChar::LineEnding(LineEnding::CrLf, 1, 6), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('i', 2, 1), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('p', 2, 2), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('s', 2, 3), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('u', 2, 4), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('m', 2, 5), chars.next().unwrap());
}

#[test]
fn _0002() {
  let mut chars = annotated("Lorem\r\nipsum".chars());
  assert_eq!(AnnotatedChar::Character('L', 1, 1), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('o', 1, 2), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('r', 1, 3), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('e', 1, 4), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('m', 1, 5), chars.next().unwrap());
  assert_eq!(AnnotatedChar::LineEnding(LineEnding::CrLf, 1, 6), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('i', 2, 1), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('p', 2, 2), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('s', 2, 3), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('u', 2, 4), chars.next().unwrap());
  assert_eq!(AnnotatedChar::Character('m', 2, 5), chars.next().unwrap());
}

#[test]
fn _0003() {
  assert_eq!("LF:1:1", annotated_to_string("\n"));
}

#[test]
fn _0004() {
  assert_eq!("CR:1:1", annotated_to_string("\r"));
}

#[test]
fn _0005() {
  assert_eq!("CRLF:1:1", annotated_to_string("\r\n"));
}

#[test]
fn _0006() {
  assert_eq!("'A':1:1", annotated_to_string("A"));
}

#[test]
fn _0007() {
  assert_eq!("'A':1:1, 'B':1:2, 'C':1:3", annotated_to_string("ABC"));
}

#[test]
fn _0008() {
  assert_eq!(
    "'L':1:1, 'o':1:2, 'r':1:3, 'e':1:4, 'm':1:5, LF:1:6, 'i':2:1, 'p':2:2, 's':2:3, 'u':2:4, 'm':2:5",
    annotated_to_string("Lorem\nipsum")
  );
}

#[test]
fn _0009() {
  assert_eq!(
    "'L':1:1, 'o':1:2, 'r':1:3, 'e':1:4, 'm':1:5, CR:1:6, 'i':2:1, 'p':2:2, 's':2:3, 'u':2:4, 'm':2:5",
    annotated_to_string("Lorem\ripsum")
  );
}

#[test]
fn _0010() {
  assert_eq!(
    "'L':1:1, 'o':1:2, 'r':1:3, 'e':1:4, 'm':1:5, CRLF:1:6, 'i':2:1, 'p':2:2, 's':2:3, 'u':2:4, 'm':2:5",
    annotated_to_string("Lorem\r\nipsum")
  );
}
