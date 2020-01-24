use crate::xsd::utils::find_child;
use crate::xsd::node_types::Extension;

pub struct ComplexContent<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> ComplexContent<'a, 'input> {

    pub fn extension(&self) -> Option<Extension<'a, 'input>> {
        find_child(&self.node, "extension").map(|node| Extension{node})
    }
}
