use roxmltree::Node;

use crate::parser::constants::attribute;
use crate::parser::parse_node;
use crate::parser::types::{Enum, EnumCase, RsEntity, TupleStruct};
use crate::parser::utils::{get_documentation, get_parent_name};
use crate::parser::xsd_elements::{ElementType, RestrictionType, XsdNode};

pub fn parse_simple_type(node: &Node, parent: &Node) -> RsEntity {
    let name = node.attr_name();

    assert_eq!(
        parent.xsd_type() == ElementType::Schema,
        name.is_some(),
        "Name required if the simpleType element is a child of the schema element, and not allowed at other times"
    );

    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect(
            "Simple types must be defined in one of the following ways: [Union, List, Restriction]",
        );

    let mut content_type = match content.xsd_type() {
        ElementType::Union => unimplemented!(), //TODO: Add union parser (No in ONVIF)
        ElementType::List => parse_node(&content, node),
        ElementType::Restriction(RestrictionType::SimpleType) => simple_type_restriction(&content),
        _ => unreachable!("Invalid content type of SimpleType {:?}", content),
    };

    if let Some(n) = name {
        content_type.set_name(n);
    }
    content_type.set_comment(get_documentation(node));
    content_type
}

fn simple_type_restriction(restriction: &Node) -> RsEntity {
    let base = restriction
        .attribute(attribute::BASE)
        .expect("The base value is required");

    let enum_cases = restriction
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "enumeration")
        .map(|n| get_enum_case(&n))
        .collect::<Vec<EnumCase>>();

    //TODO: add validators for all facet types
    if !enum_cases.is_empty() {
        RsEntity::Enum(Enum {
            comment: None,
            name: format!("{}Enum", get_parent_name(restriction)),
            cases: enum_cases,
            type_name: base.to_string(),
            subtypes: vec![],
        })
    } else {
        RsEntity::TupleStruct(TupleStruct {
            type_name: base.to_string(),
            ..Default::default()
        })
    }
}

fn get_enum_case(node: &Node) -> EnumCase {
    let value = node
        .attribute(attribute::VALUE)
        .expect("value is required attribute in enumeration facet");
    EnumCase {
        comment: get_documentation(node),
        name: value.to_string(),
        value: value.to_string(),
        type_name: None,
        type_modifiers: vec![],
    }
}

#[test]
fn test_parse_simple_type_with_list() {
    let doc = roxmltree::Document::parse(
r#"
    <xs:schema xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="http://www.onvif.org/ver10/schema">
        <xs:simpleType name="SomeType">
            <xs:annotation>
                <xs:documentation>
                    Some text
                </xs:documentation>
            </xs:annotation>
            <xs:list>
                <xs:simpleType>
                    <xs:list itemType="xs:SSD"/>
                </xs:simpleType>
            </xs:list>
        </xs:simpleType>
    </xs:schema>
"#
    ).unwrap();

    let schema = doc.root_element();
    let simple_type = find_child(&schema, "simpleType").unwrap();
    let target_ns = schema.namespaces()[0].clone();

    match parse_simple_type(&simple_type, &schema) {
        RsEntity::TupleStruct(ts) => {
            assert_eq!(ts.name, "SomeType");
            assert_eq!(ts.type_name, "xs:SSD");
            assert_eq!(
                ts.type_modifiers,
                vec![TypeModifier::Array, TypeModifier::Array]
            );
            assert_eq!(ts.comment.unwrap().trim(), "Some text");
            assert!(ts.subtypes.is_empty());
        }
        _ => unreachable!("Test failed!"),
    }
}

#[test]
fn test_parse_simple_type_with_restriction() {
    let doc = roxmltree::Document::parse(
r#"
    <xs:schema xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="http://www.onvif.org/ver10/schema">
        <xs:simpleType name="SomeType">
            <xs:annotation>
                <xs:documentation>
                    Some text
                </xs:documentation>
            </xs:annotation>
            <xs:list>
                <xs:simpleType>
                    <xs:restriction base="xs:string">
                        <xs:enumeration value="One"/>
                        <xs:enumeration value="Two"/>
                    </xs:restriction>
                </xs:simpleType>
            </xs:list>
        </xs:simpleType>
    </xs:schema>
                "#
    ).unwrap();

    let schema = doc.root_element();
    let simple_type = find_child(&schema, "simpleType").unwrap();
    let target_ns = schema.namespaces()[0].clone();

    match parse_simple_type(&simple_type, &schema) {
        RsEntity::TupleStruct(ts) => {
            assert_eq!(ts.name, "SomeType");
            assert_eq!(ts.type_name, "SomeTypeEnum");
            assert_eq!(ts.type_modifiers, vec![TypeModifier::Array]);
            assert_eq!(ts.comment.unwrap().trim(), "Some text");
            assert_eq!(ts.subtypes.len(), 1);
            match &ts.subtypes[0] {
                RsEntity::Enum(en) => {
                    assert_eq!(en.name, "SomeTypeEnum");
                    assert_eq!(en.cases.len(), 2);
                }
                _ => unreachable!("Test failed!"),
            }
        }
        _ => unreachable!("Test failed!"),
    }
}
