use roxmltree::Node;

use crate::parser::types::{RsEntity, StructField, StructFieldSource};
use crate::parser::utils::get_documentation;
use crate::parser::xsd_elements::{UseType, XsdNode};

pub fn parse_attribute(node: &Node) -> RsEntity {
    let name = node
        .attr_name()
        .or_else(|| node.attr_ref())
        .expect("All attributes have name or ref in Onvif");

    let matched_type = node
        .attr_type()
        .or_else(|| node.attr_ref())
        .unwrap_or("String");

    let type_name = match node.attr_use() {
        UseType::Optional => format!("Option<{}>", matched_type),
        UseType::Prohibited => "String".to_string(), // TODO: maybe Empty<T> or remove this field
        UseType::Required => matched_type.to_string(),
    };

    RsEntity::StructField(StructField {
        type_name,
        comment: get_documentation(node),
        subtypes: vec![],
        name: name.to_string(),
        source: StructFieldSource::Attribute,
    })
}
