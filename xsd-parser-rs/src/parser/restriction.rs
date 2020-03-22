use crate::parser::constants::attribute;
use crate::parser::types::{Enum, EnumCase, EnumSource, Facet, RsEntity, TupleStruct};
use crate::parser::utils::{get_documentation, get_parent_name};
use crate::parser::xsd_elements::{ElementType, FacetType, RestrictionType, XsdNode};
use roxmltree::Node;

pub fn parse_restriction(node: &Node, _: &Node) -> RsEntity {
    use ElementType::Restriction;
    match node.xsd_type() {
        Restriction(RestrictionType::SimpleType) => simple_type_restriction(&node),
        Restriction(RestrictionType::SimpleContent) => simple_content_restriction(&node),
        Restriction(RestrictionType::ComplexContent) => complex_content_restriction(&node),
        _ => unreachable!("Invalid restriction node: {:#?}", node),
    }
}

fn simple_type_restriction(node: &Node) -> RsEntity {
    let base = base(node);
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

fn complex_content_restriction(node: &Node) -> RsEntity {
    unimplemented!("\n{:?}\n", node)
}

fn base<'a>(node: &Node<'a, '_>) -> &'a str {
    node.attribute(attribute::BASE)
        .expect("The base value is required")
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
        .filter(|n| match n.xsd_type() {
            ElementType::Facet(FacetType::Enumeration(_)) => true,
            _ => false,
        })
        .all(|n| is_simple_enumeration(&n))
}

fn is_simple_enumeration(node: &Node) -> bool {
    node.attr_value()
        .unwrap()
        .chars()
        .all(|c| c.is_alphanumeric())
}
