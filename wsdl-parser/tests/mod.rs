use roxmltree::{Document, Node};
use wsdl_parser::generator::generate;
use wsdl_parser::parser::definitions::Definitions;
use xsd_parser::generator::builder::GeneratorBuilder;
use xsd_parser::parser::schema::parse_schema;

mod port_type_to_function;

pub fn generate_wsdl(input: &str) -> String {
    let doc = Document::parse(input).unwrap();
    let definitions = Definitions::new(&doc.root_element());
    let gen = GeneratorBuilder::default().build();
    let schemas = definitions
        .types()
        .iter()
        .flat_map(|t| t.schemas())
        .collect::<Vec<Node<'_, '_>>>();
    let mut code = schemas
        .iter()
        .map(|f| gen.generate_rs_file(&parse_schema(f)))
        .collect::<Vec<String>>();

    code.push(generate(&definitions));
    code.join("")
}

pub fn assert_ast_eq(expected: &str, actual: &str) {
    let expected = syn::parse_file(&expected).unwrap();
    let actual = syn::parse_file(&actual).unwrap();

    assert_eq!(expected, actual)
}

pub fn ast_test(input_wsdl: &str, expected_rs: &str) {
    let expected = expected_rs;
    let actual = generate_wsdl(input_wsdl);

    println!("=== expected:\n{}", expected);
    println!("=== actual:\n{}", actual);
    println!("=== diff:\n");

    text_diff::print_diff(expected, &actual, "\n");

    assert_ast_eq(expected, &actual)
}
