use crate::xsd2::complex_type::*;
use std::borrow::Cow;
use crate::xsd2::sequence::Element;
use crate::xsd2::utils::MaxOccurs;

pub fn attribute_type(attr: &Attribute, typename: Cow<str>) -> String {
    match attr.use_type() {
        UseType::Required => typename.to_string(),
        UseType::Optional => format!("Option<{}>", typename),
        UseType::Prohibited => format!("Empty<{}>", typename),
    }
}

pub fn element_type(elem: &Element, typename: Cow<str>) -> String {
    let occurs = (elem.min_occurs(), elem.max_occurs());
    let min = elem.min_occurs();
    let max = elem.max_occurs();
    if min == 0 {
        match max {
            MaxOccurs::None => format!("Option<{}>", typename),
            MaxOccurs::Unbounded => format!("Vec<{}>", typename),
            MaxOccurs::Bounded(val) => {if val > 1 {format!("Vec<{}>", typename)} else {typename.to_string()}}
        }
    }
    else if min == 1 {
        match max {
            MaxOccurs::None => typename.to_string(),
            MaxOccurs::Unbounded => format!("Vec<{}>", typename),
            MaxOccurs::Bounded(val) => {if val > 1 {format!("Vec<{}>", typename)} else {typename.to_string()}}
        }
    }
    else {
        format!("Vec<{}>", typename)
    }


}

pub fn yaserde_for_attribute(name: &str) -> String {
    format!("#[yaserde(attribute, rename = \"{}\")]", name)
}

pub fn yaserde_for_element(name: &str) -> String {
    format!("#[yaserde(rename = \"{}\")]", name)
}
