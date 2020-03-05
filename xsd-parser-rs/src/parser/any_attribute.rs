use crate::parser::types::{RsEntity, StructField, StructFieldSource, TypeModifier};
use crate::parser::utils::get_documentation;
use roxmltree::Node;

pub fn parse_any_attribute(node: &Node) -> RsEntity {
    RsEntity::StructField(StructField {
        name: "any_attribute".to_string(),
        type_name: "String".to_string(),
        comment: get_documentation(node),
        source: StructFieldSource::Attribute,
        type_modifiers: vec![TypeModifier::Empty],
        ..Default::default()
    })
}
