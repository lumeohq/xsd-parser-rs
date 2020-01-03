use std::borrow::Cow;

use crate::xsd2::sequence::{Element, Sequence};
use crate::xsd2::utils::MaxOccurs;
use crate::generator::type_tree::Types;
use crate::generator::enumeration::{EnumCase, Enum};
use crate::xsd2::choice::Choice;
use crate::generator::utils::{get_field_comment, match_type};

pub fn element_type(elem: &Element, typename: Cow<str>) -> String {
    let min = elem.min_occurs();
    let max = elem.max_occurs();
    match min {
        0 => match max {
            MaxOccurs::None => format!("Option<{}>", typename),
            MaxOccurs::Unbounded => format!("Vec<{}>", typename),
            MaxOccurs::Bounded(val) => {if val > 1 {format!("Vec<{}>", typename)} else {typename.to_string()}}
        },
        1=> match max {
            MaxOccurs::None => typename.to_string(),
            MaxOccurs::Unbounded => format!("Vec<{}>", typename),
            MaxOccurs::Bounded(val) => {if val > 1 {format!("Vec<{}>", typename)} else {typename.to_string()}}
        },
        _ => format!("Vec<{}>", typename)
    }
}

pub fn yaserde_for_attribute(name: &str) -> String {
    format!("  #[yaserde(attribute, rename = \"{}\")]\n", name)
}

pub fn yaserde_for_element(name: &str) -> String {
    format!("  #[yaserde(rename = \"{}\")]\n", name)
}

pub fn get_types_from_sequence(s: &Sequence, typename: &String, target_namespace: Option<&str>) -> Vec<Types> {
    let ch = s.choice();
    match &ch {
        Some(c) => vec!(get_type_from_choice(c, typename, target_namespace)),
        None => vec!()
    }
}

pub fn get_type_from_choice(choice: &Choice, typename: &String, target_namespace: Option<&str>) -> Types {
    let ty = match_type(typename.as_str(), target_namespace);

    Types::Enum(Enum{
        name: format!("{}Choice", ty),
        comment: String::new(),
        typename: "String".to_string(),
        cases: choice
            .elements()
            .iter()
            .map(|e| enum_case_from_element(e, target_namespace))
            .collect::<Vec<EnumCase>>()
    })
}

fn enum_case_from_element(elem: &Element, target_namespace: Option<&str>) -> EnumCase {
    EnumCase{
        name: elem.name().to_string(),
        comment: get_field_comment(elem.documentation()),
        value: "".to_string(),
        typename: Some(match_type(elem.typename(), target_namespace).to_string())
    }
}
