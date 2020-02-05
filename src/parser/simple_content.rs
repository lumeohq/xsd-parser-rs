use roxmltree::{Node, Namespace};
use crate::parser::types::{RsEntity, StructField, Struct};
use crate::parser::elements::{XmlNode, ElementType, RestrictionType, ExtensionType};
use crate::parser::utils::{match_type, get_documentation, struct_field_macros, find_child, any_attribute_field, struct_macro, attributes_to_fields};

pub fn parse_simple_content(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect("Content in simpleContent required");

    match content.xsd_type() {
        ElementType::Restriction(r) => match r {
            RestrictionType::SimpleContent => simple_content_restriction(
                &content,
                target_ns
            ),
            _ => unreachable!("Invalid restriction type of SimpleContent {:?}", r),
        },
        ElementType::Extension(e) => match e {
            ExtensionType::SimpleContent => simple_content_extension(
                &content,
                target_ns
            ),
            _ => unreachable!("Invalid extension type of SimpleContent {:?}", e),
        },
        _ => unreachable!(
            "Simple content must be defined in one of the following ways: [Restriction, Extension]"
        ),
    }
}

fn simple_content_extension(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let base = match_type(
        node.attribute("base").expect("The base value is required"),
        target_ns,
    );

    let mut fields = attributes_to_fields(node, target_ns);

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
        name: String::default(),
        subtypes: vec![],
        comment: get_documentation(node),
        macros: struct_macro(target_ns),
        fields,
    })
}

fn simple_content_restriction(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    unimplemented!("\n{:?}  {:?}\n", node, target_ns)
}


