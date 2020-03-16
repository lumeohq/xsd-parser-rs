use roxmltree::Node;

use crate::parser::constants::attribute;
use crate::parser::node_parser::parse_node;
use crate::parser::types::{
    Alias, EnumCase, EnumSource, RsEntity, Struct, StructField, StructFieldSource, TypeModifier,
};
use crate::parser::utils::get_documentation;
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

        return RsEntity::EnumCase(EnumCase {
            name: ref_attr.to_string(),
            value: String::default(),
            type_name: Some(ref_attr.to_string()),
            comment: get_documentation(element),
            type_modifiers: vec![element_modifier(element)],
            source: EnumSource::Choice,
        });
    }

    let name = element.attr_name().unwrap_or("UNSUPPORTED_ELEMENT_NAME");

    if element.has_attribute(attribute::TYPE) {
        return RsEntity::EnumCase(EnumCase {
            name: name.to_string(),
            value: String::default(),
            type_name: Some(element.attr_type().unwrap().to_string()),
            comment: get_documentation(element),
            type_modifiers: vec![element_modifier(element)],
            source: EnumSource::Choice,
        });
    }

    RsEntity::EnumCase(EnumCase {
        name: name.to_string(),
        value: String::default(),
        type_name: None,
        comment: get_documentation(element),
        type_modifiers: vec![element_modifier(element)],
        source: EnumSource::Choice,
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
        type_modifiers: vec![element_modifier(node)],
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
        .last();

    if let Some(content) = content_node {
        let mut content_entity = parse_node(&content, node);
        content_entity.set_name(name);
        return content_entity;
    }

    // No content => empty struct
    RsEntity::Struct(Struct {
        name: name.to_string(),
        comment: get_documentation(node),
        ..Default::default()
    })
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

#[cfg(test)]
mod test {
    use crate::parser::element::*;
    use crate::parser::types::RsEntity;
    use crate::parser::utils::find_child;

    #[test]
    fn test_global_element_without_type() {
        let doc = roxmltree::Document::parse(
            r#"
            <xs:schema xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="http://www.onvif.org/ver10/schema">
                <xs:element name="ChangedOnly">
                    <xs:annotation> <xs:documentation>Doc Text</xs:documentation> </xs:annotation>
                </xs:element>
            </xs:schema>
        "#).unwrap();

        let schema = doc.root_element();
        let element = find_child(&schema, "element").unwrap();

        match parse_global_element(&element) {
            RsEntity::Struct(st) => {
                assert_eq!(st.name, "ChangedOnly");
                assert_eq!(st.comment.unwrap().trim(), "Doc Text");
                assert!(st.subtypes.is_empty());
                assert!(st.fields.borrow().is_empty());
            }
            _ => unreachable!("Test failed!"),
        }
    }
}
