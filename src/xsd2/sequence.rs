use crate::xsd2::utils::{get_documentation, get_node_name, get_node_type, Node, MinMaxOccurs, Elements};
use crate::xsd2::choice::Choice;


pub struct Sequence<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> Sequence<'a, 'input> {
    pub fn any_element(&self) -> Option<AnyElement> {
        self.node.children().find(|n| n.tag_name().name() == "any").map(|n| AnyElement{node: n})
    }

    pub fn choice(&self) -> Option<Choice> {
        self.node.children().find(|n| n.tag_name().name() == "choice").map(|n| Choice{node: n})
    }
}

impl Node for Sequence<'_, '_> {
    fn node(&self) -> &roxmltree::Node {
        &self.node
    }
}

impl Elements for Sequence<'_, '_>{}

pub struct Element<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input> Element<'a, 'input> {
    pub fn name(&self) -> &'a str { get_node_name(&self.node) }
    pub fn documentation(&self) -> Option<&'a str> {
        get_documentation(&self.node)
    }
    pub fn typename(&self) -> &'a str { get_node_type(&self.node) }
}

impl Node for Element<'_, '_> {
    fn node(&self) -> &roxmltree::Node {
        &self.node
    }
}

impl MinMaxOccurs for Element<'_, '_>{}


pub struct AnyElement<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> AnyElement<'a, 'input> {
    pub fn namespace(&self) -> Option<&str> {
        self.node.attribute("namespace")
    }
}