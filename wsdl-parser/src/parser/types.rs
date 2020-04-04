use roxmltree::Node;

// Content: Sequence [1..1]
// wsdl:documentation [0..1]   from type wsdl:tDocumented
// Any element [0..*] Namespace: ##other, Process Contents: lax  from type wsdl:tExtensibleDocumented

// Attributes: None

// Used in Group wsdl:anyTopLevelOptionalElement
// Type wsdl:tDefinitions via reference to wsdl:anyTopLevelOptionalElement (Element wsdl:definitions)

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

    pub fn schemas(&self) -> Vec<Node<'_, '_>> {
        self
            .node
            .children()
            .filter_map(|n| if n.is_element() && n.tag_name().name() == "schema" {Some(n)} else {None})
            .collect()
    }
}