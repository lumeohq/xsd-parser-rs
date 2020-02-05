use crate::parser::types::{Enum, EnumCase, RsEntity, TupleStruct};
use crate::parser::utils::{
    find_child, get_documentation, get_parent_name, match_type, tuple_struct_macros,
};
use crate::parser::elements::{ElementType, RestrictionType, XmlNode};

use roxmltree::Node;

pub fn parse_simple_type(node: &Node, parent: &Node, tn: Option<&roxmltree::Namespace>) -> RsEntity {
    let name = if parent.xsd_type() == ElementType::Schema {
        node.attribute("name")
            .expect("Name required if the simpleType element is a child of the schema element")
    } else {
        get_parent_name(node)
    };

    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect(
            "Simple types must be defined in one of the following ways: [Union, List, Restriction]",
        );

    match content.xsd_type() {
        ElementType::Union => unimplemented!(), //TODO: Add union parser (No in ONVIF)
        ElementType::List => simple_type_list(&content, name, get_documentation(node), tn),
        ElementType::Restriction(r) => match r {
            RestrictionType::SimpleType => {
                simple_type_restriction(&content, name, get_documentation(node), tn)
            }
            _ => panic!("Invalid restriction type of SimpleType {:?}", r),
        },
        _ => panic!(
            "Invalid content type of SimpleType {:?}",
            content.xsd_type()
        ),
    }
}

fn simple_type_list(
    list: &Node,
    name: &str,
    doc: Option<String>,
    target_ns: Option<&roxmltree::Namespace>,
) -> RsEntity {
    let mut types: Vec<RsEntity> = vec![];

    let item_type = list.attribute("itemType");
    let type_name = if item_type.is_some() {
        item_type.unwrap()
    } else {
        let nested_simple_type = find_child(list, "simpleType").expect(
            "itemType not allowed if the content contains a simpleType element. Otherwise, required."
        );
        types.push(parse_simple_type(&nested_simple_type, list, target_ns));
        match types.last() {
            Some(ty) => ty.name(),
            None => "Unsupported_type_name",
        }
    };
    RsEntity::TupleStruct(TupleStruct {
        name: match_type(name, target_ns).to_string(),
        comment: doc,
        type_name: format!("Vec<{}>", match_type(type_name, target_ns)),
        macros: tuple_struct_macros(),
        subtypes: types,
    })
}

fn simple_type_restriction(
    restriction: &Node,
    name: &str,
    doc: Option<String>,
    target_ns: Option<&roxmltree::Namespace>,
) -> RsEntity {
    let base = match_type(
        restriction
            .attribute("base")
            .expect("The base value is required"),
        target_ns,
    );
    let struct_name = match_type(name, target_ns);

    let enum_cases = restriction
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "enumeration")
        .map(|n| get_enum_case(&n, target_ns))
        .collect::<Vec<EnumCase>>();

    //TODO: add validators for all facet types
    if !enum_cases.is_empty() {
        RsEntity::Enum(Enum {
            comment: doc,
            name: struct_name.to_string(),
            cases: enum_cases,
            type_name: base.to_string(),
            subtypes: vec![]
        })
    } else {
        RsEntity::TupleStruct(TupleStruct {
            name: struct_name.to_string(),
            comment: doc,
            type_name: base.to_string(),
            macros: tuple_struct_macros(),
            subtypes: vec![],
        })
    }
}

fn get_enum_case(node: &Node, target_ns: Option<&roxmltree::Namespace>) -> EnumCase {
    let value = node
        .attribute("value")
        .expect("value is required attribute in enumeration facet");
    EnumCase {
        comment: get_documentation(node),
        name: match_type(value, target_ns).to_string(),
        value: value.to_string(),
        type_name: None,
    }
}
