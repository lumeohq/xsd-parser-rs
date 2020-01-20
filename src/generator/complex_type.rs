use std::borrow::Cow;

use crate::xsd2::node_types::{Element, Sequence};
use crate::generator::type_tree::Types;
use crate::generator::enumeration::{EnumCase, Enum};
use crate::xsd2::node_types::Choice;
use crate::generator::utils::{get_field_comment, match_type};
use crate::xsd2::node_traits::{
    MinMaxOccurs,
    Elements,
    MaxOccurs,
    Choice as ChoiceTrait,
    Name,
    Documentation,
    TypeName
};

pub fn element_type<T: MinMaxOccurs>(elem: &T, type_name: Cow<str>) -> String {
    let min = elem.min_occurs();
    let max = elem.max_occurs();
    match min {
        0 => match max {
            MaxOccurs::None => format!("Option<{}>", type_name),
            MaxOccurs::Unbounded => format!("Vec<{}>", type_name),
            MaxOccurs::Bounded(val) => {if val > 1 {format!("Vec<{}>", type_name)} else { type_name.to_string()}}
        },
        1=> match max {
            MaxOccurs::None => type_name.to_string(),
            MaxOccurs::Unbounded => format!("Vec<{}>", type_name),
            MaxOccurs::Bounded(val) => {if val > 1 {format!("Vec<{}>", type_name)} else { type_name.to_string()}}
        },
        _ => format!("Vec<{}>", type_name)
    }
}

pub fn yaserde_for_attribute(name: &str) -> String {
    format!("  #[yaserde(attribute, rename = \"{}\")]\n", name)
}

pub fn yaserde_for_element(name: &str) -> String {
    format!("  #[yaserde(rename = \"{}\")]\n", name)
}

pub fn get_types_from_sequence(s: &Sequence, type_name: &String, target_namespace: Option<&str>) -> Vec<Types> {
    let ch = s.choice();
    match &ch {
        Some(c) => vec!(Types::Enum(get_enum_from_choice(c, type_name, target_namespace))),
        None => vec!()
    }
}

pub fn get_enum_from_choice(choice: &Choice, type_name: &String, target_namespace: Option<&str>) -> Enum {
    let ty = match_type(type_name.as_str(), target_namespace);
    Enum{
        name: format!("{}Choice", ty),
        comment: String::new(),
        type_name: "String".to_string(),
        cases: choice
            .elements()
            .iter()
            .map(|e| enum_case_from_element(e, target_namespace))
            .collect::<Vec<EnumCase>>()
    }
}

fn enum_case_from_element(elem: &Element, target_namespace: Option<&str>) -> EnumCase {
    EnumCase{
        name: elem.name().expect("NAME FOR ENUM CASE REQUIRED").to_string(),
        comment: get_field_comment(elem.documentation()),
        value: "".to_string(),
        type_name: Some(
            match_type(elem.type_name().expect("TYPE FOR ENUM CASE REQUIRED"), target_namespace).to_string())
    }
}
