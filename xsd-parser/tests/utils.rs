use xsd_parser::generator::builder::GeneratorBuilder;
use xsd_parser::parser::parse;

pub fn generate(input: &str) -> String {
    let f = parse(input).unwrap();
    let gen = GeneratorBuilder::default().build();
    gen.generate_rs_file(&f)
}

/// Checks if AST of two code fragments are equivalent.
/// Here we compare only AST, so anything not related
/// to AST is ignored, like:
///  - comments
///  - formatting
pub fn assert_ast_eq(expected: &str, actual: &str) {
    let expected = syn::parse_file(expected).unwrap();
    let actual = syn::parse_file(actual).unwrap();

    assert_eq!(expected, actual)
}

pub fn ast_test(input_xsd: &str, expected_rs: &str) {
    let expected = expected_rs;
    let actual = generate(input_xsd);

    println!("=== expected:\n{}", expected);
    println!("=== actual:\n{}", actual);
    println!("=== diff:\n");

    text_diff::print_diff(expected, &actual, "\n");

    assert_ast_eq(expected, &actual)
}
