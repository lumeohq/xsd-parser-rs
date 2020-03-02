use crate::generator::Generator;
use crate::parser::parse;
use crate::yaserde_generator::YaserdeGenerator;
use std::fmt::Write;

pub fn generate(input: &str) -> String {
    let f = parse(input).unwrap();

    let gen = YaserdeGenerator::new(&f);
    let mut output = String::new();

    for ty in f.types {
        write!(output, "{}", gen.gen_rs_entity(&ty)).unwrap();
    }

    output
}

/// Checks if AST of two code fragments are equivalent.
/// Here we compare only AST, so anything not related
/// to AST is ignored, like:
///  - comments
///  - formatting
pub fn assert_ast_eq(expected: &str, actual: &str) {
    let expected = syn::parse_file(&expected).unwrap();
    let actual = syn::parse_file(&actual).unwrap();

    assert_eq!(expected, actual)
}
