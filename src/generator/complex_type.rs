use std::borrow::Cow;

use crate::xsd2::complex_type::*;
use crate::xsd2::sequence::Element;
use crate::xsd2::utils::MaxOccurs;

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
