
pub struct Schema<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> Schema<'a, 'input> {
    pub fn target_namespace(&self) -> Option<&'a str> {
        match self.node.attribute("targetNamespace") {
            Some(tn) => self.node.
                namespaces().
                iter().
                find(|a| a.uri()==tn).
                map(|n| n.name().unwrap()),
            None => None
        }
    }
}