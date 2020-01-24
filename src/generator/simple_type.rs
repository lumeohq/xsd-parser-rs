use crate::xsd::simple_type::{FacetType, Facet, Restriction};
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

pub fn get_enum_cases(facets: &Vec<Facet>, target_namespace: Option<&roxmltree::Namespace>) -> Vec<EnumCase> {
    facets.
        iter().
        map(|f| get_enum_case(f, target_namespace)).
        collect()
}

pub fn get_enum_case(facet : &Facet, target_namespace: Option<&roxmltree::Namespace>) -> EnumCase {
    EnumCase{
        comment: get_field_comment(facet.documentation()),
        name: match_type(facet.value(), target_namespace).to_string(),
        value: facet.value().to_string(),
        type_name: None
    }
}
