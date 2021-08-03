use std::cell::RefCell;

use crate::parser::constants::tag;
use crate::parser::node_parser::parse_node;
use crate::parser::types::{
    Enum, EnumCase, EnumSource, Facet, RsEntity, Struct, StructField, StructFieldSource,
    TupleStruct,
};
use crate::parser::utils::{
    attribute_groups_to_aliases, attributes_to_fields, get_base, get_documentation, get_parent_name,
};
use crate::parser::xsd_elements::{ElementType, FacetType, RestrictionType, XsdNode};
use roxmltree::Node;

const AVAILABLE_CONTENT_TYPES: [ElementType; 7] = [
    ElementType::All, // Not presented in ONVIF
    ElementType::AnyAttribute,
    ElementType::Attribute,
    ElementType::AttributeGroup, // Not presented in ONVIF
    ElementType::Choice,         // Not presented in ONVIF
    ElementType::Group,          // Not presented in ONVIF
    ElementType::Sequence,       // Not presented in ONVIF
];

pub fn parse_restriction(node: &Node, _: &Node) -> RsEntity {
    use ElementType::Restriction;
    match node.xsd_type() {
        Restriction(RestrictionType::SimpleType) => simple_type_restriction(node),
        Restriction(RestrictionType::SimpleContent) => simple_content_restriction(node),
        Restriction(RestrictionType::ComplexContent) => complex_content_restriction(node),
        _ => unreachable!("Invalid restriction node: {:#?}", node),
    }
}

fn simple_type_restriction(node: &Node) -> RsEntity {
    let base = get_base(node);
    let facets = facets(node);

    if is_simple_enumerations(node) {
        let cases = cases(facets.as_ref());
        if !cases.is_empty() {
            return RsEntity::Enum(Enum {
                name: format!("{}Enum", get_parent_name(node)),
                cases,
                type_name: base.to_string(),
                source: EnumSource::Restriction,
                ..Default::default()
            });
        }
    };

    RsEntity::TupleStruct(TupleStruct {
        type_name: base.to_string(),
        facets,
        ..Default::default()
    })
}

fn simple_content_restriction(node: &Node) -> RsEntity {
    unimplemented!("\n{:?}\n", node)
}

// NOTE: current implementation works for types from ONVIF, but might not work
// in a general case.
fn complex_content_restriction(node: &Node) -> RsEntity {
    let base = get_base(node);
    let mut fields = attributes_to_fields(node);

    fields.push(StructField {
        name: tag::BASE.to_string(),
        type_name: base.to_string(),
        comment: get_documentation(node),
        source: StructFieldSource::Base,
        ..Default::default()
    });

    let content = node
        .children()
        .filter(|n| {
            n.is_element()
                && n.xsd_type() != ElementType::Attribute
                && AVAILABLE_CONTENT_TYPES.contains(&n.xsd_type())
        })
        .last();

    if let Some(cont) = content {
        let mut res = parse_node(&cont, node);
        if let RsEntity::Struct(s) = &mut res {
            s.comment = get_documentation(node);
            s.fields.borrow_mut().append(&mut fields);
            return res;
        }
    }

    RsEntity::Struct(Struct {
        comment: get_documentation(node),
        fields: RefCell::new(fields),
        attribute_groups: RefCell::new(attribute_groups_to_aliases(node)),
        ..Default::default()
    })
}

fn facets(node: &Node) -> Vec<Facet> {
    node.children()
        .filter_map(|n| match n.xsd_type() {
            ElementType::Facet(x) => Some(Facet {
                facet_type: x,
                comment: get_documentation(&n),
            }),
            _ => None,
        })
        .collect()
}

fn cases(facets: &[Facet]) -> Vec<EnumCase> {
    facets
        .iter()
        .filter_map(|f| match &f.facet_type {
            FacetType::Enumeration(value) => Some(EnumCase {
                comment: f.comment.clone(),
                name: value.clone(),
                value: value.clone(),
                type_name: None,
                type_modifiers: vec![],
                source: EnumSource::Restriction,
            }),
            _ => None,
        })
        .collect()
}

fn is_simple_enumerations(node: &Node) -> bool {
    node.children()
        .filter(|n| matches!(n.xsd_type(), ElementType::Facet(FacetType::Enumeration(_))))
        .all(|n| is_simple_enumeration(&n))
}

fn is_simple_enumeration(node: &Node) -> bool {
    let val = node
        .attr_value()
        .expect("Value required for xsd:enumeration");
    !val.is_empty() && val.chars().all(|c| c.is_alphanumeric() || c == '-')
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::utils::find_child;

    #[test]
    fn test_simple_type_restriction() {
        let doc = roxmltree::Document::parse(
            r#"
    <xs:schema xmlns:tt="http://www.onvif.org/ver10/schema" xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="http://www.onvif.org/ver10/schema">
        <xs:simpleType name="SomeType">
            <xs:restriction base="xs:string">
                <xs:enumeration value=""/>
            </xs:restriction>
        </xs:simpleType>
    </xs:schema>
                "#
        ).unwrap();
        let root = doc.root_element().first_element_child().unwrap();
        let restriction = find_child(&root, "restriction").unwrap();

        match simple_type_restriction(&restriction) {
            RsEntity::TupleStruct(ts) => {
                assert_eq!(ts.type_name, "xs:string");
            }
            _ => unreachable!("Test failed"),
        }
    }
}
