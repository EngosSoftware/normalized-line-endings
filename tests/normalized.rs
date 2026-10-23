use normalized_line_endings::Normalized;
use std::iter::Peekable;
use std::str::Chars;

const INPUT: &str = "Lorem \n ipsum \r dolor \r\n sit \n\r\n amet, consectetur \r\r\n\n adipiscing \n\n\r\r elit \r\n\r.";
const OUTPUT: &str =
    "Lorem \n ipsum \n dolor \n sit \n\n amet, consectetur \n\n\n adipiscing \n\n\n\n elit \n\n.";

#[test]
fn _0001() {
    assert_eq!(OUTPUT, INPUT.chars().normalized().collect::<String>());
}

#[test]
fn _0002() {
    eq(
        &mut OUTPUT.chars(),
        &mut INPUT.chars().normalized().peekable(),
    );
}

#[test]
fn _0003() {
    let input: &str = "Lorem \n ipsum \r dolor \r\n sit \n\r\n amet, consectetur \r\r\n\n adipiscing \n\n\r\r elit \r\n\r.";
    let output: &str = "Lorem \n ipsum \n dolor \n sit \n\n amet, consectetur \n\n\n adipiscing \n\n\n\n elit \n\n.";
    let mut actual = input.chars().normalized().peekable();
    let mut expected = output.chars();
    assert_eq!(expected.next().unwrap(), actual.next().unwrap());
    while actual.peek().is_some() {
        let ch = expected.next().unwrap();
        assert_eq!(ch, *actual.peek().unwrap());
        assert_eq!(ch, actual.next().unwrap());
    }
    assert!(actual.next().is_none());
    assert!(expected.next().is_none());
}

fn eq(expected: &mut Chars, actual: &mut Peekable<impl Iterator<Item = char>>) {
    assert_eq!(expected.next().unwrap(), actual.next().unwrap());
    while actual.peek().is_some() {
        let ch = expected.next().unwrap();
        assert_eq!(ch, *actual.peek().unwrap());
        assert_eq!(ch, actual.next().unwrap());
    }
    assert!(actual.next().is_none());
    assert!(expected.next().is_none());
}
