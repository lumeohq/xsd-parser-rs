use super::utils;

#[test]
fn deserialization_works() {
    mod expected {
        use macro_utils::*;
        use std::io::{Read, Write};
        use std::str::FromStr;
        use xsd_types::utils;
        use yaserde::{YaDeserialize, YaSerialize};

        trait Validate {
            fn validate(&self) -> Result<(), String>;
        }

        include!("expected.rs");
    }

    let ser = include_str!("example.xml");

    let de: expected::FooType = yaserde::de::from_str(&ser).unwrap();

    assert_eq!(de, expected::FooType("string".to_string()));
}

#[test]
fn generator_does_not_panic() {
    let rust_code = utils::generate(include_str!("input.xsd"));
    println!("{}", rust_code)
}

#[test]
#[ignore] // Validation is not needed in this case
fn generator_output_has_correct_ast() {
    let expected = include_str!("expected.rs");
    let actual = utils::generate(include_str!("input.xsd"));

    text_diff::print_diff(expected, &actual, "\n");

    utils::assert_ast_eq(expected, &actual)
}
