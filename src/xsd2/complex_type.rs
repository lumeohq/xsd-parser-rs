
use std::fmt;
use crate::xsd2::utils::*;
use crate::xsd2::sequence::Sequence;

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

