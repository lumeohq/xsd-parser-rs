use roxmltree::{Node, Namespace};
use crate::generator2::types::{RsEntity, StructField, Struct};
use crate::xsd::elements::{XmlNode, ElementType, RestrictionType, ExtensionType};
use crate::generator2::utils::{match_type, get_documentation, struct_field_macros, find_child, any_attribute_field, struct_macro, get_field_name, get_parent_name, get_fields_from_attributes};

pub fn parse_simple_content(node: &Node, parent: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name= get_parent_name(node);

    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect(
            "Simple content must be defined in one of the following ways: [Restriction, Extension]",
        );

    match content.xsd_type() {
        ElementType::Restriction(r) => match r {
            RestrictionType::SimpleContent => simple_content_restriction(
                &content,
                name,
                target_ns
            ),
            _ => unreachable!("Invalid restriction type of SimpleContent {:?}", r),
        },
        ElementType::Extension(e) => match e {
            ExtensionType::SimpleContent => simple_content_extension(
                &content,
                name,
                target_ns
            ),
            _ => unreachable!("Invalid extension type of SimpleContent {:?}", e),
        },
        _ => unreachable!(),
    }
}

fn simple_content_extension(node: &Node, name: &str, target_ns: Option<&Namespace>) -> RsEntity {
    let base = match_type(
        node.attribute("base").expect("The base value is required"),
        target_ns,
    );

    let struct_name = match_type(name, target_ns);

    let mut fields = get_fields_from_attributes(node, target_ns);

    fields.push(
        StructField {
            name: "base".to_string(),
            type_name: base.to_string(),
            comment: get_documentation(node),
            macros: struct_field_macros("base"),
            subtypes: vec![]
        }
    );

    match find_child(node, "anyAttribute") {
        Some(_) => fields.push(any_attribute_field()),
        None => (),
    };

    RsEntity::Struct(Struct {
        name: struct_name.to_string(),
        subtypes: vec![],
        comment: get_documentation(node),
        macros: struct_macro(target_ns),
        fields,
    })
}

fn simple_content_restriction(node: &Node, name: &str, target_ns: Option<&Namespace>) -> RsEntity {
    unimplemented!()
}


