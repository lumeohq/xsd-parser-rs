use roxmltree::Node;

use crate::parser::node_parser::parse_node;
use crate::parser::types::RsEntity;
use crate::parser::xsd_elements::{ElementType, XsdNode};

pub fn parse_simple_content(node: &Node) -> RsEntity {
    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect("Content in simpleContent required");

    parse_node(&content, node)
}
