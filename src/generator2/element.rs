use roxmltree::Node;
use roxmltree::Namespace;

use crate::generator2::generator::parse_node;
use crate::generator2::types::{Alias, RsEntity, Struct, StructField};
use crate::generator2::utils::{get_documentation, match_type, struct_macro, get_field_name, get_field_comment, struct_field_macros};
use crate::xsd::elements::{ElementType, XmlNode, min_occurs, max_occurs, MaxOccurs};
use std::borrow::Cow;

pub fn parse_element(node: &Node, parent: &Node, target_ns: Option<&roxmltree::Namespace>) -> RsEntity {
    match parent.xsd_type() {
        ElementType::Schema => parse_global_element(node, parent, target_ns),
        ElementType::Sequence => parse_field_of_sequence(node, target_ns),
        _ => element_default(node, target_ns)
    }
}

fn element_default(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let ty = node.attribute("type").unwrap_or("UNSUPPORTED");
    RsEntity::Alias(
        Alias {
            name: match_type("UNSUPPORTED", target_ns).into(),
            original: match_type(ty, target_ns).into(),
            comment: get_documentation(node),
            subtypes: vec![],
        }
    )
}

fn parse_field_of_sequence(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name = node.attribute("name").unwrap_or("UNSUPPORTED_ELEMENT_NAME");

    let type_name = element_type(
        node,
        match_type(
            node.attribute("type").unwrap_or("UNSUPPORTED_TYPE_OF_ELEMENT"),
            target_ns
        )
    );


    RsEntity::StructField(
        StructField{
            name: get_field_name(name),
            type_name,
            comment: get_documentation(node),
            macros: struct_field_macros(name),
            subtypes: vec![]
        }
    )
}

fn parse_global_element(node: &Node, parent: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name = node
        .attribute("name")
        .expect("Name required if the element is a child of the schema");

    if node.has_attribute("type") {
        return RsEntity::Alias(
            Alias {
                name: match_type(name, target_ns).into(),
                original: match_type(node.attribute("type").unwrap(), target_ns).into(),
                comment: get_documentation(node),
                subtypes: vec![],
            }
        );
    }
    let content_node = node.children().filter(|n| n.is_element()).last().expect("MUST HAVE CONTENT");
    let content = parse_node(&content_node, node, target_ns);
    println!("{:?}", node);
    println!("{:?}", content_node);


    RsEntity::Struct(
        Struct {
            name: match_type(name, target_ns).into(),
            fields: vec![],
            comment: get_documentation(node),
            subtypes: vec![content],
            macros: struct_macro(target_ns),
        }
    )
}

pub fn element_type(node: &Node, type_name: Cow<str>) -> String {
    let min = min_occurs(node);
    let max = max_occurs(node);
    match min {
        0 => match max {
            MaxOccurs::None => format!("Option<{}>", type_name),
            MaxOccurs::Unbounded => format!("Vec<{}>", type_name),
            MaxOccurs::Bounded(val) => {
                if val > 1 {
                    format!("Vec<{}>", type_name)
                } else {
                    type_name.to_string()
                }
            }
        },
        1=> match max {
            MaxOccurs::None => type_name.to_string(),
            MaxOccurs::Unbounded => format!("Vec<{}>", type_name),
            MaxOccurs::Bounded(val) => {
                if val > 1 {
                    format!("Vec<{}>", type_name)
                } else {
                    type_name.to_string()
                }
            }
        },
        _ => format!("Vec<{}>", type_name)
    }
}


