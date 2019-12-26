use crate::xsd2::utils::{MaxOccurs, MinOccurs, get_documentation, get_node_name, get_node_type, max_occurs, min_occurs};


pub struct Sequence<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> Sequence<'a, 'input> {

    pub fn max_occurs(&self) -> MaxOccurs { max_occurs(&self.node) }
    pub fn min_occurs(&self) -> MinOccurs { min_occurs(&self.node) }

    pub fn elements(&self) -> Vec<Element> {
        self.node.
            children().
            filter(|node| node.is_element() && node.tag_name().name() == "element").
            map(|node| Element{node}).
            collect::<Vec<Element>>()
    }

    pub fn any_element(&self) -> Option<AnyElement> {
        self.node.children().find(|n| n.tag_name().name() == "any").map(|n| AnyElement{node: n})
    }

}

pub struct Element<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input> Element<'a, 'input> {
    pub fn name(&self) -> &'a str { get_node_name(&self.node) }
    pub fn documentation(&self) -> Option<&'a str> {
        get_documentation(&self.node)
    }
    pub fn typename(&self) -> &'a str { get_node_type(&self.node) }

    pub fn max_occurs(&self) -> MaxOccurs { max_occurs(&self.node) }
    pub fn min_occurs(&self) -> MinOccurs { min_occurs(&self.node) }
}

pub struct AnyElement<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> AnyElement<'a, 'input> {

    pub fn max_occurs(&self) -> MaxOccurs { max_occurs(&self.node) }
    pub fn min_occurs(&self) -> MinOccurs { min_occurs(&self.node) }

    pub fn namespace(&self) -> Option<&str> {
        self.node.attribute("namespace")
    }
}