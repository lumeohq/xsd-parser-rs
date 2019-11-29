use std::fs;
use std::io::Read;

mod xsd;
pub use xsd::simple_type::SimpleType;
pub use xsd::complex_type::ComplexType;
use roxmltree::Node;
use crate::xsd::utils::{find_child, get_struct_comment};


fn main() {
    let text = load_file("xsd/onvif.xsd");
    let doc = match roxmltree::Document::parse(&text) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error: {}.", e);
            return;
        },
    };
    let root = doc.root();
    let schema = find_child(&root, "schema").expect("All xsd need schema element");
    generate_and_print(&schema);
}

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn generate_and_print(schema: &Node) {
    for child in schema.children().filter(|node| !node.is_text()) {
        if child.is_comment() {
            print!("{}", get_struct_comment(child.text()));
        }
        else if child.is_element() {
            match child.tag_name().name() {
                "simpleType" => {print!("{:?}", SimpleType::new(&child));},
                "complexType" => {print!("{:?}", ComplexType::new(&child));},
                _ => {println!("\nUNSUPPORTED_ELEMENT: {:?}", child);}
            }
        }
    }
}
