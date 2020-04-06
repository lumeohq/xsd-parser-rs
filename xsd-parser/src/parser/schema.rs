use roxmltree::Node;

use crate::parser::node_parser::parse_node;
use crate::parser::types::RsFile;
use crate::parser::utils::target_namespace;
use crate::parser::xsd_elements::{XsdNode, ElementType};

pub fn parse_schema<'input>(schema: &Node<'_, 'input>) -> RsFile<'input> {
    RsFile {
        name: "".into(),
        namespace: None,
        target_ns: target_namespace(&schema).cloned(),
        xsd_ns: schema
            .namespaces()
            .iter()
            .find(|a| a.uri() == "http://www.w3.org/2001/XMLSchema")
            .cloned(),
        types: schema
            .children()
            .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
            .map(|node| parse_node(&node, schema))
            .collect(),
    }
}
