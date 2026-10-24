use normalized_line_endings::Normalized;

#[test]
fn _0001() {
    assert_eq!("-\n", "-\n".chars().normalized().collect::<String>());
}

#[test]
fn _0002() {
    assert_eq!("-\n", "-\r".chars().normalized().collect::<String>());
}

#[test]
fn _0003() {
    assert_eq!("-\n", "-\r\n".chars().normalized().collect::<String>());
}

#[test]
fn _0004() {
    assert_eq!(
        "Lorem \n ipsum",
        "Lorem \n ipsum".chars().normalized().collect::<String>()
    );
}

#[test]
fn _0005() {
    assert_eq!(
        "Lorem \n ipsum",
        "Lorem \r ipsum".chars().normalized().collect::<String>()
    );
}

#[test]
fn _0006() {
    assert_eq!(
        "Lorem \n ipsum",
        "Lorem \r\n ipsum".chars().normalized().collect::<String>()
    );
}

#[test]
fn _0007() {
    assert_eq!(
        "Lorem \n\n ipsum",
        "Lorem \n\n ipsum".chars().normalized().collect::<String>()
    );
}

#[test]
fn _0008() {
    assert_eq!(
        "Lorem \n\n ipsum",
        "Lorem \n\r ipsum".chars().normalized().collect::<String>()
    );
}

#[test]
fn _0009() {
    assert_eq!(
        "Lorem \n\n ipsum",
        "Lorem \n\r\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0010() {
    assert_eq!(
        "Lorem \n\n ipsum",
        "Lorem \r\r ipsum".chars().normalized().collect::<String>()
    );
}

#[test]
fn _0011() {
    assert_eq!(
        "Lorem \n\n ipsum",
        "Lorem \r\r\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0012() {
    assert_eq!(
        "Lorem \n\n ipsum",
        "Lorem \r\n\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0013() {
    assert_eq!(
        "Lorem \n\n ipsum",
        "Lorem \r\n\r ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0014() {
    assert_eq!(
        "Lorem \n\n ipsum",
        "Lorem \r\n\r\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0015() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \n\n\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0016() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \n\n\r ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0017() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \n\n\r\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0018() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \n\r\r ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0019() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \n\r\r\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0020() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \n\r\n\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0021() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \n\r\n\r ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0022() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \n\r\n\r\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0023() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\r\r ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0024() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\r\r\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0025() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\r\n\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0026() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\r\n\r ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0027() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\r\n\r\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0028() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\n\n\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0029() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\n\n\r ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0030() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\n\n\r\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0031() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\n\r\r ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0032() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\n\r\r\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0033() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\n\r\n\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0034() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\n\r\n\r ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0035() {
    assert_eq!(
        "Lorem \n\n\n ipsum",
        "Lorem \r\n\r\n\r\n ipsum"
            .chars()
            .normalized()
            .collect::<String>()
    );
}

#[test]
fn _0036() {
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
