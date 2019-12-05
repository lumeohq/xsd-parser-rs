use std::fs;
use std::io::{Read};

mod xsd;
pub use xsd::simple_type::SimpleType;
pub use xsd::complex_type::ComplexType;
use roxmltree::Node;
use crate::xsd::utils::{find_child, get_struct_comment};

mod xsd2;
mod generator;
pub use xsd2::simple_type::SimpleType as ST;
pub use generator::simple_type::simple_type;
use inflector::cases::pascalcase::to_pascal_case;

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
    generate_and_print2(&schema);
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
                "simpleType" => {
                    print!("{:?}", SimpleType::new(&child));
                },
                "complexType" => {print!("{:?}", ComplexType::new(&child));},
                _ => {println!("\nUNSUPPORTED_ELEMENT: {:?}", child);}
            }
        }
    }
}



fn generate_and_print2(schema: &Node) {
    let stypes: Vec<ST> = schema.
        children().
        filter(|node| node.is_element() && node.tag_name().name() == "simpleType").
        map(|node| ST{node}).collect();

    for st in stypes {
        println!("{}", simple_type(&st));
    }
}