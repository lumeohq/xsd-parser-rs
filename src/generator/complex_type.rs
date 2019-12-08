use crate::xsd2::complex_type::*;
use inflector::cases::snakecase::to_snake_case;
use std::borrow::Cow;
use crate::generator::utils::{get_comment};

pub fn field_from_attribute(attr: &Attribute) -> String {
    let name = attr.name();
    format!("  {}\n  pub {}: {},  {}",
            yaserde_attributes(name),
            to_snake_case(&name),
            attr.typename(),
            get_comment(attr.documentation())
    )
}

pub fn yaserde_attributes(name: &str) -> String {
    format!("#[yaserde(attribute, rename = \"{}\")]", name)
}
