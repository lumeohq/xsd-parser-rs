use roxmltree::Node;

use crate::parser::types::{RsEntity, StructField, StructFieldSource, TypeModifier};
use crate::parser::utils::get_documentation;
use crate::parser::xsd_elements::{UseType, XsdNode};

pub fn parse_attribute(node: &Node) -> RsEntity {
    let name = node
        .attr_name()
        .or_else(|| node.attr_ref())
        .expect("All attributes have name or ref")
        .to_string();

    let type_name = node
        .attr_type()
        .or_else(|| node.attr_ref())
        .unwrap_or("String")
        .to_string();

    let type_modifier = match node.attr_use() {
        UseType::Optional => TypeModifier::Option,
        UseType::Prohibited => TypeModifier::Empty,
        UseType::Required => TypeModifier::None,
    };

    RsEntity::StructField(StructField {
        type_name,
        comment: get_documentation(node),
        subtypes: vec![],
        name,
        source: StructFieldSource::Attribute,
        type_modifiers: vec![type_modifier],
    })
}
