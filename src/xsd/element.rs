extern crate inflector;
use inflector::cases::snakecase::to_snake_case;

use crate::xsd::utils::*;
use crate::xsd::traits::*;
use std::collections::HashMap;


pub struct Element {
    prefix: Option<String>,
    name: String,
    type_: String,
    documentation: Option<String>
}

impl Element {
    pub fn new(node: & roxmltree::Node) -> Self {
        Element {
            prefix: get_node_namespace(node),
            name: get_node_name(node),
            type_: get_element_type(node),
            documentation: get_documentation(node).map(|t| t.to_string())
        }
    }

    pub fn generate_yaserde_attributes(&self) -> String {
        match &self.prefix {
            None => format!("#[yaserde(rename = \"{}\")]", self.name),
            Some(prefix) => format!("#[yaserde(prefix = \"{}\", rename = \"{}\")]",
                                    prefix,
                                    self.name,
            )
        }
    }
}

impl GenerateCode for Element {
    fn generate_code(&self) -> String {
        format!("  {}\n  pub {}: {},  {}",
                self.generate_yaserde_attributes(),
                to_snake_case(&self.name),
                self.type_,
                get_field_comment(&self.documentation),
        )
    }
}

fn get_element_type(node: & roxmltree::Node) -> String {
    let type_ = get_node_type(node);

    match node.attribute("minOccurs").and_then(|s| s.parse::<u32>().ok()) {
        Some(v) => match v {
            0 => format!("Option<{}>", type_),
            1 => type_,
            _ => format!("Vec<{}>", type_),
        },
        None => match node.attribute("maxOccurs") {
            Some("unbounded") => format!("Vec<{}>", type_),
            Some(s) => match s.parse::<u32>().ok() {
                Some(v) => match v {
                    0 => format!("Empty<{}>", type_),
                    1 => type_,
                    _ => format!("Vec<{}>", type_),
                },
                None => type_
            }
            _ => type_
        }
    }
}