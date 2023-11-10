use super::utils;

#[test]
fn deserialization_works() {
    mod expected {
        use xsd_parser::generator::validator::Validate;
        use yaserde_derive::{YaDeserialize, YaSerialize};

        include!("expected.rs");
    }

    let ser = include_str!("example.xml");

    let de: expected::Section = yaserde::de::from_str(ser).unwrap();

    assert_eq!(de, expected::Section { 
        section_choice: expected::section::SectionChoice::Tocheading("2D Structure".into()),
        description: Some("Structure depictions of this compound, including computationally generated two-dimensional (2D) and three-dimensional (3D) structures, as well as experimentally determined 3D single-crystal structures.".into()),
        url: None,
    });
}

#[test]
fn generator_does_not_panic() {
    println!("{}", utils::generate(include_str!("input.xsd")))
}

#[test]
fn generator_output_has_correct_ast() {
    utils::ast_test(include_str!("input.xsd"), include_str!("expected.rs"));
}
