use crate::xsd2::complex_type::*;
use std::borrow::Cow;

pub fn attribute_type(attr: &Attribute, typename: Cow<str>) -> String {
    match attr.use_type() {
        UseType::Required => typename.to_string(),
        UseType::Optional => format!("Option<{}>", typename),
        UseType::Prohibited => format!("Empty<{}>", typename),
    }
}

pub fn yaserde_attributes(name: &str) -> String {
    format!("#[yaserde(attribute, rename = \"{}\")]", name)
}
