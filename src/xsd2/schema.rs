pub struct Schema<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> Schema<'a, 'input> {
    pub fn target_namespace(&self) -> Option<&'a roxmltree::Namespace<'input>> {
        match self.node.attribute("targetNamespace") {
            Some(tn) => {
                self.node.
                    namespaces().
                    iter().
                    find( | a| a.uri() == tn)
            },
            None => None
        }
    }
}
