use roxmltree::Node;

use crate::parser::constants::attribute;
use crate::parser::parser::parse_node;
use crate::parser::types::{Alias, EnumCase, RsEntity, StructField, StructFieldSource, TypeModifier};
use crate::parser::utils::{get_documentation, get_type_name};
use crate::parser::xsd_elements::{max_occurs, min_occurs, ElementType, MaxOccurs, XsdNode};

const SUPPORTED_CONTENT_TYPES: [ElementType; 2] =
    [ElementType::SimpleType, ElementType::ComplexType];

pub fn parse_element(node: &Node, parent: &Node) -> RsEntity {
    match parent.xsd_type() {
        ElementType::Schema => parse_global_element(node),
        ElementType::Sequence => parse_field_of_sequence(node, parent),
        ElementType::Choice => parse_case_of_choice(node),
        _ => element_default(node),
    }
}

fn element_default(node: &Node) -> RsEntity {
    let ty = node.attr_type().unwrap_or("UNSUPPORTED");
    RsEntity::Alias(Alias {
        name: "UNSUPPORTED".into(),
        original: ty.into(),
        comment: get_documentation(node),
        subtypes: vec![],
    })
}

fn parse_case_of_choice(element: &Node) -> RsEntity {
    if element.has_attribute(attribute::REF) {
        let ref_attr = element.attr_ref().unwrap();
        let name = ref_attr.split(':').last().unwrap();
        let type_name = element_type(element, ref_attr);

        return RsEntity::EnumCase(EnumCase {
            name: name.to_string(),
            value: String::default(),
            type_name: Some(type_name),
            comment: get_documentation(element),
            //subtypes: vec![]
        });
    }

    let name = element.attr_name().unwrap_or("UNSUPPORTED_ELEMENT_NAME");

    if element.has_attribute(attribute::TYPE) {
        let type_name = element_type(element, element.attr_type().unwrap());
        return RsEntity::EnumCase(EnumCase {
            name: name.to_string(),
            value: String::default(),
            type_name: Some(type_name),
            comment: get_documentation(element),
            //subtypes: vec![]
        });
    }

    let type_name = element_type(element, element.attr_type().unwrap_or("String"));

    RsEntity::EnumCase(EnumCase {
        name: get_type_name(name),
        value: String::default(),
        type_name: Some(type_name),
        comment: get_documentation(element),
        //subtypes: vec![]
    })
}

fn parse_field_of_sequence(node: &Node, _: &Node) -> RsEntity {
    let name = node
        .attr_name()
        .unwrap_or_else(|| node.attr_ref().unwrap_or("UNSUPPORTED_ELEMENT_NAME"))
        .to_string();

    if node.has_attribute(attribute::TYPE) || node.has_attribute(attribute::REF) {
        let type_name = node
            .attr_type()
            .unwrap_or_else(|| node.attr_ref().unwrap_or("String"))
            .to_string();

        return RsEntity::StructField(StructField {
            name,
            type_name,
            comment: get_documentation(node),
            source: StructFieldSource::Element,
            type_modifiers: vec![element_modifier(node)],
            ..Default::default()
        });
    }

    let content_node = node
        .children()
        .filter(|n| SUPPORTED_CONTENT_TYPES.contains(&n.xsd_type()))
        .last()
        .unwrap_or_else(|| {
            panic!(
                "Must have content if no 'type' or 'ref' attribute: {:?}",
                node
            )
        });

    let mut field_type = parse_node(&content_node, node);

    field_type.set_name(format!("{}Type", name).as_str());

    RsEntity::StructField(StructField {
        name,
        type_name: field_type.name().to_string(),
        comment: get_documentation(node),
        subtypes: vec![field_type],
        source: StructFieldSource::Element,
        type_modifiers: vec![element_modifier(node)]
    })
}

fn parse_global_element(node: &Node) -> RsEntity {
    let name = node
        .attr_name()
        .expect("Name required if the element is a child of the schema");

    if node.has_attribute(attribute::TYPE) {
        return RsEntity::Alias(Alias {
            name: name.into(),
            original: node.attr_type().unwrap().into(),
            comment: get_documentation(node),
            subtypes: vec![],
        });
    }

    let content_node = node
        .children()
        .filter(|n| SUPPORTED_CONTENT_TYPES.contains(&n.xsd_type()))
        .last()
        .expect("MUST HAVE CONTENT");

    let mut content = parse_node(&content_node, node);
    content.set_name(name);
    content
}

pub fn element_type(node: &Node, type_name: &str) -> String {
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
        1 => match max {
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
        _ => format!("Vec<{}>", type_name),
    }
}

pub fn element_modifier(node: &Node) -> TypeModifier {
    let min = min_occurs(node);
    let max = max_occurs(node);
    match min {
        0 => match max {
            MaxOccurs::None => TypeModifier::Option,
            MaxOccurs::Unbounded => TypeModifier::Array,
            MaxOccurs::Bounded(val) => {
                if val > 1 {
                    TypeModifier::Array
                } else {
                    TypeModifier::None
                }
            }
        },
        1 => match max {
            MaxOccurs::None => TypeModifier::None,
            MaxOccurs::Unbounded => TypeModifier::Array,
            MaxOccurs::Bounded(val) => {
                if val > 1 {
                    TypeModifier::Array
                } else {
                    TypeModifier::None
                }
            }
        },
        _ => TypeModifier::Array,
    }
}
