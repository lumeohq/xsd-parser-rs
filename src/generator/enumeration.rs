use std::borrow::Cow;
use crate::xsd2::simple_type::Facet;
use crate::generator::utils::{get_comment, get_type_name};

pub fn enum_struct(name: &String, facets: &Vec<Facet>, typename: Cow<str>) -> String {
    format!(
r#"pub enum {name} {{
{cases}
  __Unknown__({typename})
}}

impl {name} {{
  pub fn new(s: &str) -> Self {{
    match s {{
{match_lines}
      value => Self::__Unknown__(value.to_string()),
    }}
  }}
}}"#,
    name=name,
    cases=enum_cases(facets),
    typename=typename,
    match_lines=emun_matches(facets))
}


fn enum_case(facet : &Facet) -> String {
    format!("{}  {},",
            get_comment(facet.documentation()),
            get_type_name(facet.value())
    )
}

fn enum_cases(facets: &Vec<Facet>) -> String {
    facets.
        iter().
        map(|f| enum_case(f)).
        collect::<Vec<String>>().join("\n")
}

fn enum_match(facet : &Facet) -> String {
    format!("      \"{}\" => Self::{},",
            facet.value(),
            get_type_name(facet.value())
    )
}

fn emun_matches(facets: &Vec<Facet>) -> String {
        facets.
        iter().
        map(|f| enum_match(f)).
        collect::<Vec<String>>().join("\n")
}

