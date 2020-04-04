use crate::parser::constants::attribute;
use crate::parser::port_type::PortType;
use crate::parser::{ElementType, WsdlElement};
use roxmltree::{Namespace, Node, Document};
use std::collections::HashMap;
use crate::parser::binding::Binding;

#[derive(Debug)]
pub struct Definitions<'a> {
    node: Node<'a, 'a>,
    imports: Vec<Import<'a>>,
    schemas: Vec<Node<'a, 'a>>,
    messages: HashMap<&'a str, Vec<Part<'a>>>,
    port_types: Vec<PortType<'a>>,
    bindings: Vec<Binding<'a>>,
    //TODO: services
}

impl<'a> Definitions<'a> {
    pub fn target_namespace(&self) -> Option<&'a Namespace<'_>> {
        match self.node().attribute(attribute::TARGET_NAMESPACE) {
            Some(tn) => self.node().namespaces().iter().find(|a| a.uri() == tn),
            None => None,
        }
    }

    pub fn node(&self) -> &Node<'_, '_> {
        &self.node
    }


    pub fn name(&self) -> Option<&str> {
        self.node().attribute(attribute::NAME)
    }

    pub fn imports(&self) -> &[Import] {
        self.imports.as_ref()
    }

    pub fn schemas(&self) -> &[Node] {
        self.schemas.as_ref()
    }

    pub fn messages(&self) -> &HashMap<&'a str, Vec<Part<'a>>> {
        &self.messages
    }

    pub fn new(definitions: &Node<'a, '_>) -> Self {
        let mut imports = vec![];
        let mut schemas = vec![];
        let mut messages = HashMap::new();
        let mut port_types = vec![];
        let mut bindings = vec![];
        for ch in definitions.children().filter(|n| n.is_element()) {
            match ch.wsdl_type() {
                ElementType::Import => imports.push(Import::new(&ch)),
                ElementType::Types => schemas.push(
                    ch.first_child()
                        .expect("Schema required in wsdl:types element"),
                ),
                ElementType::Message => insert_message(&ch, &mut messages),
                ElementType::PortType => port_types.push(PortType::new(&ch)),
                ElementType::Binding => bindings.push(Binding::new(&ch)),
                _ => {}
            }
        }
        Self {
            imports,
            schemas,
            messages,
            port_types,
            bindings,
            node: definitions.clone()
        }
    }

}

fn insert_message<'a, 'input>(
    node: &Node<'a, 'input>,
    messages: &mut HashMap<&'a str, Vec<Part<'a>>>,
) {
    let res = messages.insert(
        node.attribute(attribute::NAME)
            .expect("Name required for wsdl:messge"),
        node.children()
            .filter(|n| n.wsdl_type() == ElementType::Part)
            .map(|n| Part::new(&n))
            .collect(),
    );
    assert!(res.is_none(), "Message name must be unique");
}

#[derive(Clone, Debug)]
pub struct Import<'a> {
    node: Node<'a, 'a>,
}

impl<'a> Import<'a> {
    pub fn namespace(&self) -> &'a str {
        self.node
            .attribute(attribute::NAMESPACE)
            .expect("Namespace required for wsdl:Import")
    }

    pub fn location(&self) -> &'a str {
        self.node
            .attribute(attribute::LOCATION)
            .expect("Location required for wsdl:Import")
    }

    pub fn new(node: &Node<'a, '_>) -> Self {
        Self { node: node.clone() }
    }
}

#[derive(Clone, Debug)]
pub struct Part<'a> {
    node: Node<'a, 'a>,
}

impl<'a> Part<'a> {
    pub fn name(&self) -> &'a str {
        self.node
            .attribute(attribute::NAME)
            .expect("Name required for wsdl:part")
    }

    pub fn element(&self) -> Option<&'a str> {
        self.node.attribute(attribute::ELEMENT)
    }

    pub fn type_(&self) -> Option<&'a str> {
        self.node.attribute(attribute::TYPE)
    }

    pub fn new(node: &Node<'a, '_>) -> Self {
        Self { node: node.clone() }
    }
}
