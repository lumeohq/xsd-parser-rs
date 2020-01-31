use crate::generator2::types::{RsEntity, StructField, Struct};
use roxmltree::{Namespace, Node};
use crate::generator2::utils::{match_type, get_fields_from_attributes, get_documentation, struct_field_macros, find_child, any_attribute_field, struct_macro};
use crate::xsd::elements::{XmlNode, ElementType, RestrictionType, ExtensionType};

pub fn parse_complex_content(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect(
            "Content in complexContent required",
        );

    match content.xsd_type() {
        ElementType::Restriction(r) => match r {
            RestrictionType::ComplexContent => complex_content_restriction(
                &content,
                target_ns
            ),
            _ => unreachable!("Invalid restriction type of SimpleContent {:?}", r),
        },
        ElementType::Extension(e) => match e {
            ExtensionType::ComplexContent => complex_content_extension(
                &content,
                target_ns
            ),
            _ => unreachable!("Invalid extension type of SimpleContent {:?}", e),
        },
        _ => unreachable!(
            "Complex content must be defined in one of the following ways: [Restriction, Extension]"
        ),
    }
}

fn complex_content_extension(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let base = match_type(
        node.attribute("base").expect("The base value is required"),
        target_ns,
    );

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

    RsEntity::Struct(
        Struct {
            name: String::default(),
            subtypes: vec![],
            comment: get_documentation(node),
            macros: struct_macro(target_ns),
            fields,
        }
    )
}

fn complex_content_restriction(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    unimplemented!("\n{:?}  {:?}\n", node, target_ns)
}
