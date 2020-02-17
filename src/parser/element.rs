use std::borrow::Cow;

use roxmltree::Namespace;
use roxmltree::Node;

use crate::parser::constants::attribute;
use crate::parser::parser::parse_node;
use crate::parser::types::{Alias, EnumCase, RsEntity, StructField, StructFieldSource};
use crate::parser::utils::{
    get_documentation, get_field_name, get_type_name, match_type, yaserde_for_element,
};
use crate::parser::xsd_elements::{max_occurs, min_occurs, ElementType, MaxOccurs, XsdNode};

const SUPPORTED_CONTENT_TYPES: [ElementType; 2] =
    [ElementType::SimpleType, ElementType::ComplexType];

pub fn parse_element(
    node: &Node,
    parent: &Node,
    target_ns: Option<&roxmltree::Namespace>,
) -> RsEntity {
    match parent.xsd_type() {
        ElementType::Schema => parse_global_element(node, target_ns),
        ElementType::Sequence => parse_field_of_sequence(node, parent, target_ns),
        ElementType::Choice => parse_case_of_choice(node, target_ns),
        _ => element_default(node, target_ns),
    }
}

fn element_default(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let ty = node.attr_type().unwrap_or("UNSUPPORTED");
    RsEntity::Alias(Alias {
        name: match_type("UNSUPPORTED", target_ns).into(),
        original: match_type(ty, target_ns).into(),
        comment: get_documentation(node),
        subtypes: vec![],
    })
}

fn parse_case_of_choice(element: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    if element.has_attribute(attribute::REF) {
        let ref_attr = element.attr_ref().unwrap();
        let name = ref_attr.split(':').last().unwrap();
        let type_name = element_type(element, match_type(ref_attr, target_ns));

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
        let type_name = element_type(element, match_type(element.attr_type().unwrap(), target_ns));
        return RsEntity::EnumCase(EnumCase {
            name: name.to_string(),
            value: String::default(),
            type_name: Some(type_name),
            comment: get_documentation(element),
            //subtypes: vec![]
        });
    }

    let type_name = element_type(
        element,
        match_type(
            element.attr_type().unwrap_or("UNSUPPORTED_TYPE_OF_ELEMENT"),
            target_ns,
        ),
    );

    RsEntity::EnumCase(EnumCase {
        name: get_type_name(name),
        value: String::default(),
        type_name: Some(type_name),
        comment: get_documentation(element),
        //subtypes: vec![]
    })
}

fn parse_field_of_sequence(node: &Node, _: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name = node
        .attr_name()
        .unwrap_or_else(|| node.attr_ref().unwrap_or("UNSUPPORTED_ELEMENT_NAME"));

    if node.has_attribute(attribute::TYPE) || node.has_attribute(attribute::REF) {
        let type_name = element_type(
            node,
            match_type(
                node.attr_type()
                    .unwrap_or_else(|| node.attr_ref().unwrap_or("UNSUPPORTED_TYPE_OF_ELEMENT")),
                target_ns,
            ),
        );
        return RsEntity::StructField(StructField {
            name: get_field_name(name),
            type_name,
            comment: get_documentation(node),
            macros: yaserde_for_element(name, target_ns),
            subtypes: vec![],
            source: StructFieldSource::Element
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

    let mut field_type = parse_node(&content_node, node, target_ns);

    field_type.set_name(match_type(format!("{}Type", name).as_str(), target_ns).as_ref());

    RsEntity::StructField(StructField {
        name: get_field_name(name),
        type_name: element_type(node, field_type.name().into()),
        comment: get_documentation(node),
        macros: yaserde_for_element(name, target_ns),
        subtypes: vec![field_type],
        source: StructFieldSource::Element
    })
}

fn parse_global_element(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name = node
        .attr_name()
        .expect("Name required if the element is a child of the schema");

    if node.has_attribute(attribute::TYPE) {
        return RsEntity::Alias(Alias {
            name: match_type(name, target_ns).into(),
            original: match_type(node.attr_type().unwrap(), target_ns).into(),
            comment: get_documentation(node),
            subtypes: vec![],
        });
    }

    let content_node = node
        .children()
        .filter(|n| SUPPORTED_CONTENT_TYPES.contains(&n.xsd_type()))
        .last()
        .expect("MUST HAVE CONTENT");

    let mut content = parse_node(&content_node, node, target_ns);
    content.set_name(match_type(name, target_ns).as_ref());
    content
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
