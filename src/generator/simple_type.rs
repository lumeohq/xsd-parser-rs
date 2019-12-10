use crate::xsd2::simple_type::{FacetType, Facet, Restriction};
use std::borrow::Cow;
use crate::generator::type_tree::EnumCase;
use crate::generator::utils::{get_structure_comment, get_type_name, get_field_comment};


pub fn tuple_struct(doc: &String, name: &String, typename: Cow<str>) -> String {
    format!("{}pub struct {} ({}); \n\n", doc, name, typename)
}

pub fn list_simple_type(doc: &String, name: &String, item_type: &str) -> String {
    tuple_struct(
        doc,
        name,
        Cow::Owned(format!("Vec<{}>", item_type))
    )
}

pub fn get_enum_facets<'a, 'input>(restriction: &Restriction<'a, 'input>) -> Vec<Facet<'a, 'input>> {
    restriction.facets().
    into_iter().
    filter(|f| match f.facet_type {
        FacetType::Enumeration => true,
        _ => false
    }).collect()
}

pub fn enum_case2(facet : &Facet) -> EnumCase {
    EnumCase{
        comment: get_field_comment(facet.documentation()),
        name: get_type_name(facet.value()),
        value: facet.value().to_string()
    }
}

pub fn enum_cases2(facets: &Vec<Facet>) -> Vec<EnumCase> {
    facets.
        iter().
        map(|f| enum_case2(f)).
        collect()
}