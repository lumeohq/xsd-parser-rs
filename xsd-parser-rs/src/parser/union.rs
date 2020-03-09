use roxmltree::Node;

use crate::parser::constants::attribute;
use crate::parser::node_parser::parse_node;
use crate::parser::types::{Enum, EnumCase, RsEntity, Struct};
use crate::parser::utils::{
    attributes_to_fields, enum_to_field, find_child, get_documentation, get_parent_name,
};
use crate::parser::xsd_elements::{ElementType, XsdNode};
use std::cell::RefCell;

pub fn parse_union(union: &Node) -> RsEntity {
    let mut cases = union
        .attribute(attribute::MEMBER_TYPES)
        .map(|s| create_enum_cases(s))
        .unwrap_or_else(|| vec![]);

    let subtypes = union
        .children()
        .filter(|e| e.is_element() && e.xsd_type() == ElementType::SimpleType)
        .map(|st| parse_node(&st, union))
        .collect::<Vec<RsEntity>>();

    cases.append(
        &mut subtypes
            .iter()
            .map(|rs_entity| EnumCase {
                name: rs_entity.name().to_string(),
                type_name: Some(rs_entity.name().to_string()),
                ..Default::default()
            })
            .collect(),
    );

    let mut union_enum = Enum {
        cases,
        subtypes,
        comment: get_documentation(union),
        type_name: "String".into(),
        ..Default::default()
    };

    let mut fields = attributes_to_fields(union);

    if fields.is_empty() {
        RsEntity::Enum(union_enum)
    } else {
        union_enum.name = format!("{}Choice", get_parent_name(union));
        fields.push(enum_to_field(union_enum));
        RsEntity::Struct(Struct {
            fields: RefCell::new(fields),
            ..Default::default()
        })
    }
}

fn create_enum_cases(member_types: &str) -> Vec<EnumCase> {
    member_types
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|mt| EnumCase {
            name: mt.to_string(),
            type_name: Some(mt.to_string()),
            ..Default::default()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::parser::types::RsEntity;
    use crate::parser::union::{create_enum_cases, parse_union};
    use crate::parser::utils::find_child;

    #[test]
    fn test_create_enum() {
        let cases = create_enum_cases("Type1 Type2  Type3");
        assert_eq!(cases.len(), 3);
        assert_eq!(cases[0].name, "Type1");
    }

    #[test]
    fn test_parse_union() {
        let doc = roxmltree::Document::parse(
            r#"
    <xs:schema xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="http://www.onvif.org/ver10/schema">
        <xs:simpleType name="SomeType">
            <xs:annotation><xs:documentation>Some text</xs:documentation></xs:annotation>
            <xs:union memberTypes="Type1 Type2" />
        </xs:simpleType>
    </xs:schema>
    "#
        ).unwrap();

        let simple_type = find_child(&doc.root_element(), "simpleType").unwrap();
        let union = find_child(&simple_type, "union").unwrap();

        let result = parse_union(&union);

        match result {
            RsEntity::Enum(en) => {
                assert_eq!(en.cases.len(), 2);
                assert_eq!(en.cases[0].name, "Type1");
                assert_eq!(en.cases[1].name, "Type2");
                assert_eq!(en.name, String::default());
            }
            _ => unreachable!("Test Failed"),
        }
    }
}
