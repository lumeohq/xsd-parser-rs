use roxmltree::Node;

use crate::generator2::generator::parse_node;
use crate::generator2::types::{Alias, RsType, Struct};
use crate::generator2::utils::{get_documentation, match_type, struct_macro};
use crate::xsd::elements::{ElementType, XmlNode};

pub fn parse_element(node: &Node, parent: &Node, target_ns: Option<&roxmltree::Namespace>) -> RsType {
    if parent.xsd_type() == ElementType::Schema {
        return parse_global_element(node, parent, target_ns);
    }

    let ty = node.attribute("type").unwrap_or("UNSUPPORTED");

    RsType::Alias(
        Alias {
            name: match_type("UNSUPPORTED", target_ns).into(),
            original: match_type(ty, target_ns).into(),
            comment: get_documentation(node),
            subtypes: vec![],
        }
    )
}

fn parse_global_element(node: &Node, parent: &Node, target_ns: Option<&roxmltree::Namespace>) -> RsType {
    let name = node
        .attribute("name")
        .expect("Name required if the element is a child of the schema");

    if node.has_attribute("type") {
        return RsType::Alias(
            Alias {
                name: match_type(name, target_ns).into(),
                original: match_type(node.attribute("type").unwrap(), target_ns).into(),
                comment: get_documentation(node),
                subtypes: vec![],
            }
        );
    }
    let content_node = node.children().filter(|n| n.is_element()).last().expect("MUST HAVE CONTENT");
    let content = parse_node(&content_node, parent, target_ns);
    println!("{:?}", node);
    println!("{:?}", content_node);


    RsType::Struct(
        Struct {
            name: match_type(name, target_ns).into(),
            fields: vec![],
            comment: get_documentation(node),
            subtypes: vec![content],
            macros: struct_macro(target_ns),
        }
    )
}



