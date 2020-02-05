use crate::parser::types::{RsEntity, Enum};
use roxmltree::Node;
use crate::parser::elements::{ElementType, XmlNode};
use crate::parser::parser::parse_node;

pub fn parse_choice(choice: &Node, target_ns: Option<&roxmltree::Namespace>) -> RsEntity {
    let enum_cases = choice
        .children()
        .filter(|n| n.is_element() && n.xsd_type() == ElementType::Element)
        .map(|n| match parse_node(&n, choice, target_ns) {
            RsEntity::EnumCase(case) => case,
            _ => unreachable!("Elements in choice must be a enum variants")
        })
        .collect();

    RsEntity::Enum(
        Enum{
            name: "".to_string(),
            cases: enum_cases,
            comment: None,
            type_name: "String".to_string(),
            subtypes: vec![]
        }
    )
}