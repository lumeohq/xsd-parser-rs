use crate::xsd2::simple_type::{FacetType, Facet, Restriction};
use std::borrow::Cow;
use crate::generator::utils::{get_field_comment, match_type};
use crate::generator::enumeration::EnumCase;


pub fn get_enum_facets<'a, 'input>(restriction: &Restriction<'a, 'input>) -> Vec<Facet<'a, 'input>> {
    restriction.facets().
    into_iter().
    filter(|f| match f.facet_type {
        FacetType::Enumeration => true,
        _ => false
    }).collect()
}

pub fn enum_case2(facet : &Facet, target_namespace: Option<&str>) -> EnumCase {
    EnumCase{
        comment: get_field_comment(facet.documentation()),
        name: match_type(facet.value(), target_namespace).to_string(),
        value: facet.value().to_string(),
        typename: None
    }
}

pub fn enum_cases2(facets: &Vec<Facet>, target_namespace: Option<&str>) -> Vec<EnumCase> {
    facets.
        iter().
        map(|f| enum_case2(f, target_namespace)).
        collect()
}