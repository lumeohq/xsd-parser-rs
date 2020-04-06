use crate::tests::{generate_wsdl, ast_test};

#[test]
fn generator_does_not_panic() {
    println!("{}", generate_wsdl(include_str!("input.xsd")))
}

#[test]
fn generator_output_has_correct_ast() {
    ast_test(include_str!("input.xsd"), include_str!("expected.rs"));
}
