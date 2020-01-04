use std::fmt;

use crate::xsd2::node_types::{Attribute, Choice, Sequence, SimpleContent};
use crate::xsd2::complex_content::ComplexContent;
use crate::xsd2::utils::*;

pub struct ComplexType<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> ComplexType<'a, 'input> {

    pub fn name(&self) -> Option<&'a str> {
        self.node.attribute("name")
    }

    pub fn documentation(&self) -> Option<&'a str> {
        get_documentation(&self.node)
    }

    pub fn attributes(&self) -> Vec<Attribute> {
        self.node.children().
            filter(|e| e.is_element() && e.tag_name().name() == "attribute").
            map(|e| Attribute{node: e.clone()}).collect()
    }

    pub fn sequence(&self) -> Option<Sequence> {
        find_child(&self.node, "sequence").map(|node| Sequence{node})
    }

    pub fn complex_content(&self) -> Option<ComplexContent> {
        find_child(&self.node, "complexContent").map(|node| ComplexContent{node})
    }

    pub fn simple_content(&self) -> Option<SimpleContent> {
        find_child(&self.node, "simpleContent").map(|node| SimpleContent{node})
    }

    pub fn choice(&self) -> Option<Choice> {
        find_child(&self.node, "choice").map(|node| Choice{node})
    }

    pub fn has_any_attribute(&self) -> bool {
        find_child(&self.node, "anyAttribute").is_some()
    }
}

impl<'a, 'input> fmt::Display for ComplexType<'a, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}  //{:?}", self.name(), self.documentation())
    }
}



