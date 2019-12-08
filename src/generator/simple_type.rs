use crate::xsd2::simple_type::{FacetType, Facet, Restriction};
use std::borrow::Cow;


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