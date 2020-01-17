pub struct Schema<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

pub struct TargetNamespace<'a> {
    pub uri: &'a str,
    pub prefix: &'a str,
}

impl<'a, 'input: 'a> Schema<'a, 'input> {
    pub fn target_namespace(&self) -> Option<TargetNamespace<'a>> {
        match self.node.attribute("targetNamespace") {
            Some(tn) => {
                let prefix=self.node.
                    namespaces().
                    iter().
                    find( | a| a.uri() == tn).
                    map( | n| n.name().unwrap());
                match prefix {
                    Some(p) => Some(TargetNamespace{uri: tn, prefix: p}),
                    None => None
                }
            },
            None => None
        }
    }
}
