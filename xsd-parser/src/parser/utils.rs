use std::str;

extern crate inflector;

use roxmltree::{Namespace, Node};

use crate::parser::constants::attribute;
use crate::parser::node_parser::parse_node;
use crate::parser::types::{Enum, RsEntity, StructField, StructFieldSource, TypeModifier};
use crate::parser::xsd_elements::{ElementType, XsdNode};

pub fn target_namespace<'a, 'input>(node: &Node<'a, 'input>) -> Option<&'a Namespace<'input>> {
    match node.attribute(attribute::TARGET_NAMESPACE) {
        Some(tn) => node.namespaces().iter().find(|a| a.uri() == tn),
        None => None,
    }
}

pub fn find_child<'a, 'input>(node: &Node<'a, 'input>, tag_name: &str) -> Option<Node<'a, 'input>> {
    node.children()
        .find(|e| e.is_element() && e.tag_name().name() == tag_name)
}

pub fn get_documentation<'a>(node: &Node<'a, '_>) -> Option<String> {
    find_child(node, "annotation")
        .and_then(|node| find_child(&node, "documentation"))
        .and_then(|node| node.text().map(|s| s.to_string()))
}

pub fn get_parent_name<'a>(node: &Node<'a, '_>) -> &'a str {
    match node.parent_element() {
        Some(parent) => {
            if parent.xsd_type() == ElementType::Schema {
                return "SchemaElement";
            }

            match parent.attribute(attribute::NAME) {
                Some(s) => s,
                None => get_parent_name(&parent),
            }
        }
        None => "UnsupportedName",
    }
}

pub fn get_base<'a>(node: &Node<'a, '_>) -> &'a str {
    node.attribute(attribute::BASE)
        .expect("The base value is required")
}

pub fn attributes_to_fields(node: &Node) -> Vec<StructField> {
    node.children()
        .filter(|n| {
            n.xsd_type() == ElementType::Attribute || n.xsd_type() == ElementType::AnyAttribute
        })
        .map(|n| match parse_node(&n, node) {
            RsEntity::StructField(sf) => sf,
            _ => unreachable!("Invalid attribute parsing: {:?}", n),
        })
        .collect()
}

pub fn enum_to_field(en: Enum) -> StructField {
    StructField {
        name: en.name.clone(),
        type_name: en.name.clone(),
        subtypes: vec![RsEntity::Enum(en)],
        source: StructFieldSource::Element,
        ..Default::default()
    }
}

pub fn elements_to_fields(node: &Node, parent_name: &str) -> Vec<StructField> {
    node.children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .map(|n| match parse_node(&n, node) {
            RsEntity::StructField(mut sf) => {
                if sf.type_name.ends_with(parent_name) {
                    sf.type_modifiers.push(TypeModifier::Recursive)
                }
                sf
            }
            RsEntity::Enum(mut en) => {
                en.name = format!("{}Choice", parent_name);
                enum_to_field(en)
            }
            _ => unreachable!("\nError: {:?}\n{:?}", n, parse_node(&n, node)),
        })
        .collect()
}
