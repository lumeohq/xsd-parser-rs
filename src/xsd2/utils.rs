use roxmltree::*;

pub fn find_child<'a, 'input>(node: &Node<'a, 'input>, tag_name: &str) -> Option<Node<'a, 'input>> {
    node.children().find(|e| e.is_element() && e.tag_name().name() == tag_name)
}

pub fn find_element<'a, 'input>(node: &Node<'a, 'input>, tag_name: &str) -> Option<Node<'a, 'input>> {
    match node.
        traverse().
        find(|e| match e {
            Edge::Open(x) => x.is_element() && x.tag_name().name() == tag_name,
            _ => false
        })
    {
        Some(Edge::Open(node)) => Some(node.clone()),
        _ => None
    }
}

pub fn get_documentation<'a>(node: &Node<'a, '_>) -> Option<&'a str> {
    match find_element(node, "documentation") {
        Some(node) => node.text(),
        None => None
    }
}