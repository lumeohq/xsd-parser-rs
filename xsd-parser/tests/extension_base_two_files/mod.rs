use super::utils;

#[test]
fn deserialization_works() {
    mod expected {
        use yaserde_derive::{YaDeserialize, YaSerialize};
        include!("expected.rs");
    }

    let ser = include_str!("example.xml");

    let de: expected::FooType = yaserde::de::from_str(ser).unwrap();

    assert_eq!(
        de,
        expected::FooType {
            a: 150.0,
            b: 3,
            c: "string".to_string(),
        }
    );
}

#[test]
fn generator_does_not_panic() {
    println!("{}", utils::generate(include_str!("input.xsd")))
}

#[test]
#[ignore]
fn generator_output_has_correct_ast() {
    utils::ast_test(include_str!("input.xsd"), include_str!("expected.rs"));
}
