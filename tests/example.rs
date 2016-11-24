extern crate antlr;

#[test]
fn lexer() {
    assert_eq!(antlr::example::lexer::serialized_lexer_atn[0], 3);
    antlr::DEFAULT_ATN_DESERIALIZER.deserialize(antlr::example::lexer::serialized_lexer_atn);
}
