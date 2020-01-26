use roxmltree::Namespace;

use crate::generator2::complex_type::parse_complex_type;
use crate::generator2::element::parse_element;
use crate::generator2::simple_type::parse_simple_type;
use crate::generator2::types::RsType;
use crate::generator2::utils::target_namespace;
use crate::xsd::elements::{ElementType, XmlNode};

pub fn parse(text: &str) {
    let doc = match roxmltree::Document::parse(&text) {
        Ok(doc) => doc,
        Err(e) => {
            panic!("Error: {}.", e);
        }
    };

    for ty in parse_nodes(&doc.root(), None) {
        println!("{}", ty)
    }
    //TODO: add return value
}

pub fn parse_nodes(parent_node: &roxmltree::Node<'_, '_>, tn: Option<&Namespace>) -> Vec<RsType> {
    let mut types: Vec<RsType> = vec![];

    for node in parent_node.children().filter(|n| n.is_element()) {
        if node.xsd_type() == ElementType::Schema {
            types.append(&mut parse_nodes(&node, target_namespace(&node)));
        } else {
            types.push(parse_node(&node, parent_node, target_namespace(&node)));
        }
    }
    types
}

pub fn parse_node(node: &roxmltree::Node<'_, '_>, parent: &roxmltree::Node, tn: Option<&Namespace>) -> RsType {
    use ElementType::*;
    match node.xsd_type() {
        SimpleType => parse_simple_type(&node, parent, tn),
        ComplexType => parse_complex_type(&node, parent, tn),
        Element => parse_element(&node, parent, tn),
        _ => unreachable!(),
    }
}
