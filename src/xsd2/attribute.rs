use std::borrow::Cow;

use crate::xsd2::utils::{get_documentation, get_node_name, get_node_type};

pub struct Attribute<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input> Attribute<'a, 'input> {
    pub fn name(&self) -> &'a str { get_node_name(&self.node) }
    pub fn documentation(&self) -> Option<&'a str> {
        get_documentation(&self.node)
    }
    pub fn typename(&self) -> &'a str { get_node_type(&self.node) }

    pub fn use_type(&self) -> UseType {
        match self.node.attribute("use") {
            Some(a) => match a {
                "optional" => UseType::Optional,
                "Prohibited" => UseType::Prohibited,
                "required" => UseType::Required,
                _ => UseType::Optional
            },
            None => UseType::Optional
        }
    }
}

pub enum UseType {
    Optional,
    Prohibited,
    Required,
}


pub fn attribute_type(attr: &Attribute, typename: Cow<str>) -> String {
    match attr.use_type() {
        UseType::Required => typename.to_string(),
        UseType::Optional => format!("Option<{}>", typename),
        UseType::Prohibited => format!("Empty<{}>", typename),
    }
}