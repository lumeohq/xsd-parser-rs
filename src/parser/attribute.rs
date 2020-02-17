use roxmltree::{Namespace, Node};

use crate::parser::types::{RsEntity, StructField, StructFieldSource};
use crate::parser::utils::{get_documentation, get_field_name, match_type, yaserde_for_attribute};
use crate::parser::xsd_elements::{UseType, XsdNode};

pub fn parse_attribute(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name = node
        .attr_name()
        .or_else(|| node.attr_ref())
        .expect("All attributes have name or ref in Onvif");

    let matched_type = match_type(
        node.attr_type()
            .or_else(|| node.attr_ref())
            .unwrap_or("String"),
        target_ns,
    );

    let type_name = match node.attr_use() {
        UseType::Optional => format!("Option<{}>", matched_type),
        UseType::Prohibited => "String".to_string(), // TODO: maybe Empty<T> or remove this field
        UseType::Required => matched_type.to_string(),
    };

    RsEntity::StructField(StructField {
        macros: yaserde_for_attribute(name),
        type_name,
        comment: get_documentation(node),
        subtypes: vec![],
        name: get_field_name(name),
        source: StructFieldSource::Attribute
    })
}
