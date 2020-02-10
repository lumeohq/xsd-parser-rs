use roxmltree::{Namespace, Node};

use crate::parser::types::{RsEntity, StructField};
use crate::parser::utils::{get_documentation, get_field_name, match_type, struct_field_macros};
use crate::parser::xsd_elements::{UseType, XsdNode};

pub fn parse_attribute(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name = node
        .attr_name()
        .or(node.attr_ref())
        .expect("All attributes have name or ref in Onvif");

    let matched_type = match_type(
        node.attr_type().or(node.attr_ref()).unwrap_or("()"),
        target_ns,
    );

    let type_name = match node.attr_use() {
        UseType::Optional => format!("Option<{}>", matched_type),
        UseType::Prohibited => "()".to_string(), // TODO: maybe Empty<T> or remove this field
        UseType::Required => matched_type.to_string(),
    };

    RsEntity::StructField(StructField {
        macros: struct_field_macros(name),
        type_name,
        comment: get_documentation(node),
        subtypes: vec![],
        name: get_field_name(name),
    })
}
