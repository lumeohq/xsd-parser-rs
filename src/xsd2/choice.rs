use crate::xsd2::utils::{MaxOccurs, MinOccurs, max_occurs, min_occurs};
use crate::xsd2::sequence::Element;

pub struct Choice<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> Choice<'a, 'input> {

    pub fn max_occurs(&self) -> MaxOccurs { max_occurs(&self.node) }
    pub fn min_occurs(&self) -> MinOccurs { min_occurs(&self.node) }

    pub fn elements(&self) -> Vec<Element> {
        self.node.
            children().
            filter(|node| node.is_element() && node.tag_name().name() == "element").
            map(|node| Element{node}).
            collect::<Vec<Element>>()
    }
}
