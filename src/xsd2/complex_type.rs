
use std::fmt;
use crate::xsd2::utils::*;
use crate::xsd2::sequence::Sequence;
use crate::xsd2::complex_content::ComplexContent;
use crate::xsd2::simple_content::SimpleContent;
use crate::xsd2::choice::Choice;

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

pub enum UseType {
    Optional,
    Prohibited,
    Required,
}

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
                _=> UseType::Optional
            },
            None => UseType::Optional
        }
    }
}

