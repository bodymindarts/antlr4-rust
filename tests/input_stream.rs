extern crate antlr;

#[test]
fn look_ahead() {
    let is = antlr::InputStream::new("1+2^3");
    assert_eq!(is.la(-1), -1)
}
