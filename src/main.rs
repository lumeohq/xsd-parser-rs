use std::fs;
use std::io::Read;

mod xsd;
pub use xsd::simple_type::xsd::SimpleType;
pub use xsd::complex_type::xsd::ComplexType;
use roxmltree::Edge;




fn main() {
    let text = load_file("xsd/onvif.xsd");
    let doc = match roxmltree::Document::parse(&text) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error: {}.", e);
            return;
        },
    };
    let schema = doc.root();



   //pretty_print_code(&schema);
}

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn pretty_print(node: &roxmltree::Node, shift: usize) {
    print!("{}>{:?}\n", "-".repeat(shift), node);
    for child in node.children().filter(|e| !e.is_text()) {
        pretty_print(&child, shift+2);
    }
}

fn pretty_print_code(node: &roxmltree::Node) {
    if node.is_comment() {
        let comment = node.text().unwrap_or("").trim();
        if comment.len() > 2 {
            println!("//{}", comment);
        }
    }
    else if node.is_element() {
        if node.tag_name().name() == "simpleType" {
            print!("{:?}", SimpleType::new(node));
        }
        else if node.tag_name().name() == "complexType" {
            print!("{:?}", ComplexType::new(node));
        }
    }
    for child in node.children().filter(|e| !e.is_text()) {
        pretty_print_code(&child);
    }
}