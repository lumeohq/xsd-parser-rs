use crate::xsd2::sequence::Sequence;
use crate::xsd2::utils::{find_child, AnyAttribute};

pub struct Extension<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> Extension<'a, 'input> {
    pub fn sequence(&self) -> Option<Sequence<'a, 'input>> {
        find_child(&self.node, "sequence").map(|node| Sequence{node})
    }

    pub fn any_attribute(&self) -> Option<AnyAttribute> {
        find_child(&self.node, "anyAttribute")
    }

    pub fn base(&self) -> &'a str {
        self.node.attribute("base").expect("base required attribute for extension")
    }

}
