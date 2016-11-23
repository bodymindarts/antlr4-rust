extern crate antlr;

use antlr::IntStream;
use antlr::CharStream;

#[test]
fn look_ahead() {
    let is = antlr::InputStream::new("1+2^3");

    assert_eq!(is.la(0), 0);
    assert_eq!(is.la(-1), -1);
    assert_eq!(is.la(6), antlr::EOF);
    assert_eq!(is.la(2), "1+2^3".as_bytes()[1] as i32);
}

#[test]
fn consume() {
    let mut is = antlr::InputStream::new("1");
    is.consume();
}
#[test]
#[should_panic(expected = "cannot consume EOF")]
fn consume_panic() {
    let mut is = antlr::InputStream::new("1");
    is.consume();
    is.consume();
}

#[test]
fn get_text() {
    let is = antlr::InputStream::new("1+2^3");
    assert_eq!(is.get_text(antlr::Interval::new(1, 3)), "+2^");
}
