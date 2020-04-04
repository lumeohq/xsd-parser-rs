use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct Types<'a> {
    node: Node<'a, 'a>,
}

impl<'a> Types<'a> {
    pub fn new(node: &Node<'a, '_>) -> Self {
        Self {
            node: node.clone()
        }
    }
}