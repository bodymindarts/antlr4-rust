extern crate antlr;

#[test]
fn constants() {
    assert_eq!(antlr::SERIALIZED_VERSION, 3);
    assert_eq!(antlr::SUPPORTED_UUIDS[0],
               "AADB8D7E-AEEF-4415-AD2B-8204D6CF042E")
}
