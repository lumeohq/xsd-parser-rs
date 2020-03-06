use roxmltree::Node;

use crate::parser::node_parser::parse_node;
use crate::parser::types::RsFile;
use crate::parser::utils::target_namespace;

pub fn parse_schema<'input>(schema: &Node<'_, 'input>) -> RsFile<'input> {
    RsFile {
        name: "".into(),
        namespace: None,
        target_ns: target_namespace(&schema).cloned(),
        types: schema
            .children()
            .filter(|n| n.is_element())
            .map(|node| parse_node(&node, schema))
            .collect(),
    }
}
