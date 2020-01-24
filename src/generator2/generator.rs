use crate::generator2::complex_type::parse_complex_type;
use crate::generator2::simple_type::parse_simple_type;
use crate::generator2::types::RsType;
use crate::generator2::utils::target_namespace;
use crate::xsd::elements::{ElementType, XmlNode};
use roxmltree::Namespace;

pub fn parse(text: &str) {
    let doc = match roxmltree::Document::parse(&text) {
        Ok(doc) => doc,
        Err(e) => {
            panic!("Error: {}.", e);
        }
    };

    for ty in parse_node(&doc.root(), None) {
        println!("{}", ty)
    }
    //TODO: add return value
}

fn parse_node(parent_node: &roxmltree::Node<'_, '_>, tn: Option<&Namespace>) -> Vec<RsType> {
    use ElementType::*;
    let mut types: Vec<RsType> = vec![];

    for node in parent_node.children().filter(|n| n.is_element()) {
        match node.xsd_type() {
            Schema => types.append(&mut parse_node(&node, target_namespace(&node))),
            SimpleType => types.push(parse_simple_type(&node, parent_node, tn)),
            ComplexType => types.push(parse_complex_type(&node, parent_node, tn)),
            _ => (),
        };
    }
    types
}
