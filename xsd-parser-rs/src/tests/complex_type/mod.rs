use super::utils;

#[test]
fn deserialization_works() {
    mod expected {
        use std::io::{Read, Write};
        use yaserde::{YaDeserialize, YaSerialize};

        include!("expected.rs");
    }

    let ser = include_str!("example.xml");

    let de: expected::FooType = yaserde::de::from_str(&ser).unwrap();

    assert_eq!(de, expected::FooType { min: 1, max: 2 });
}

#[test]
fn generator_does_not_panic() {
    let rust_code = utils::generate(include_str!("input.xsd"));
    println!("{}", rust_code)
}

#[test]
fn generator_output_has_correct_ast() {
    let expected = include_str!("expected.rs");
    let actual = utils::generate(include_str!("input.xsd"));

    text_diff::print_diff(expected, &actual, "\n");

    utils::assert_ast_eq(expected, &actual)
}
