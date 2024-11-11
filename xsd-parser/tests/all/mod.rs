use super::utils;

#[test]
fn deserialization_works() {
    mod expected {
        use xsd_parser::generator::validator::Validate;
        use yaserde::{YaDeserialize, YaSerialize};

        include!("expected.rs");
    }

    let ser = include_str!("example.xml");

    let de: expected::FooType = yaserde::de::from_str(ser).unwrap();

    assert_eq!(
        de,
        expected::FooType {
            once: 222,
            optional: None,
            once_specify: 444,
            twice_or_more: vec![111, 333, 555]
        }
    );
}

#[test]
fn generator_does_not_panic() {
    println!("{}", utils::generate(include_str!("input.xsd")))
}

#[test]
fn generator_output_has_correct_ast() {
    utils::ast_test(include_str!("input.xsd"), include_str!("expected.rs"));
}
