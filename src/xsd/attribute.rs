use crate::xsd::utils::*;
use crate::xsd::traits::*;

pub struct Attribute {
    name: String,
    type_: String,
    documentation: Option<String>,
}

impl Attribute {
    pub fn new(node: & roxmltree::Node) -> Self {
        Attribute {
            name: get_node_name(node),
            type_: get_attribute_type(node),
            documentation: get_documentation(node).map(|v| v.to_string()),
        }
    }
}

impl GenerateCode for Attribute {
    fn generate_code(&self) -> String {
        format!("  {}: {},  {}",
                self.name,
                self.type_,
                get_field_comment(&self.documentation)
        )
    }
}

fn get_attribute_type(node: & roxmltree::Node) -> String {
    let type_ = get_node_type(node);
    match node.attribute("use") {
        Some("required") => type_,
        Some("optional") => format!("Option<{}>", type_),
        _ => format!("Empty<{}>", type_),
    }
}