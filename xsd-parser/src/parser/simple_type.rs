use roxmltree::Node;

use crate::parser::node_parser::parse_node;
use crate::parser::types::RsEntity;
use crate::parser::utils::get_documentation;
use crate::parser::xsd_elements::{ElementType, XsdNode};

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

    let mut content_type = parse_node(&content, node);

    if let Some(n) = name {
        content_type.set_name(n);
    }
    content_type.set_comment(get_documentation(node));
    content_type
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::types::TypeModifier;
    use crate::parser::utils::find_child;

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

    #[test]
    fn test_parse_simple_type_with_complex_enumerations_in_restriction() {
        let doc = roxmltree::Document::parse(
            r#"
    <xs:schema xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="http://www.onvif.org/ver10/schema">
        <xs:simpleType name="SomeType">
            <xs:restriction base="xs:string">
                <xs:enumeration value="xns:One"/>
                <xs:enumeration value="Two"/>
            </xs:restriction>
        </xs:simpleType>
    </xs:schema>
                "#
        ).unwrap();

        let schema = doc.root_element();
        let simple_type = find_child(&schema, "simpleType").unwrap();
        match parse_simple_type(&simple_type, &schema) {
            RsEntity::TupleStruct(ts) => {
                assert_eq!(ts.name, "SomeType");
                assert_eq!(ts.type_name, "xs:string");
                assert!(ts.type_modifiers.is_empty());
                assert!(ts.comment.is_none());
                assert!(ts.subtypes.is_empty());
                assert_eq!(ts.facets.len(), 2)
            }
            _ => unreachable!("Test failed!"),
        }
    }
}
