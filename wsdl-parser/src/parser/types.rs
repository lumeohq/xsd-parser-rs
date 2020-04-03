use roxmltree::{Node, Namespace};
use crate::parser::constants::attribute;
use std::collections::HashMap;
use roxmltree::NodeType::Element;
use crate::parser::{ElementType, WsdlElement};


#[derive(Clone, Debug)]
pub struct Definitions<'a, 'input: 'a> {
    node: Node<'a, 'input>,
    imports: Vec<Import<'a, 'input>>,
    schemas: Vec<Node<'a, 'input>>,
    messages: HashMap<&'a str, Vec<Part<'a, 'input>>>,
    //port_types:
}


impl<'a, 'input: 'a> Definitions<'a, 'input>{
    pub fn target_namespace(&self) -> Option<&'a Namespace<'input>> {
        match self.node.attribute(attribute::TARGET_NAMESPACE) {
            Some(tn) => self.node.namespaces().iter().find(|a| a.uri() == tn),
            None => None,
        }
    }

    pub fn name(&self) -> Option<&'a str> {
        self.node.attribute(attribute::NAME)
    }

    pub fn imports(&self) -> &[Import] {
        self.imports.as_ref()
    }

    pub fn schemas(&self) -> &[Node] {
        self.schemas.as_ref()
    }

    pub fn messages(&self) -> &HashMap<&'a str, Vec<Part<'a, 'input>>> {
        &self.messages
    }

    pub fn new(node: &Node<'a, 'input>) -> Self {
        let mut imports = vec![];
        let mut schemas = vec![];
        let mut messages = HashMap::new();
        for ch in node.children().filter(|n| n.is_element()) {
            match ch.wsdl_type() {
                ElementType::Import=> { imports.push(Import::new(&ch)); },
                ElementType::Types => { schemas.push(ch.first_child().expect("Schema required in wsdl:types element") )},
                ElementType::Message => { insert_message(&ch, &mut messages) },

                _ => {}
            }
        }
        Self {
            node: node.clone(),
            imports,
            schemas,
            messages,
        }

    }
}

fn insert_message<'a, 'input>(node: &Node<'a, 'input>, messages: &mut HashMap<&'a str, Vec<Part<'a, 'input>>>) {
    let res = messages.insert(
        node.attribute(attribute::NAME).expect("Name required for wsdl:messge"),
        node
            .children()
            .filter(|n| n.wsdl_type() == ElementType::Part)
            .map(|n| Part::new(&n))
            .collect()
    );
    assert!(res.is_none(), "Message name must be unique");
}

#[derive(Clone, Debug)]
pub struct Import<'a, 'input: 'a> {
    pub node: Node<'a, 'input>
}

impl<'a, 'input: 'a> Import<'a, 'input>{
    pub fn namespace(&self) -> &'a str {
        self.node.attribute(attribute::NAMESPACE).expect("Namespace required for wsdl:Import")
    }

    pub fn location(&self) -> &'a str {
        self.node.attribute(attribute::LOCATION).expect("Location required for wsdl:Import")
    }

    pub fn new(node: &Node<'a, 'input>) -> Self {
        Self {
            node: node.clone()
        }
    }
}


#[derive(Clone, Debug)]
pub struct Part<'a, 'input: 'a> {
    pub node: Node<'a, 'input>
}

impl<'a, 'input: 'a> Part<'a, 'input>{
    pub fn name(&self) -> &'a str {
        self.node.attribute(attribute::NAME).expect("Name required for wsdl:part")
    }

    pub fn element(&self) -> Option<&'a str> {
        self.node.attribute(attribute::ELEMENT)
    }

    pub fn type_(&self) -> Option<&'a str> {
        self.node.attribute(attribute::TYPE)
    }

    pub fn new(node: &Node<'a, 'input>) -> Self {
        Self {
            node: node.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub struct PortType<'a, 'input: 'a> {
    pub node: Node<'a, 'input>
}

impl<'a, 'input: 'a> PortType<'a, 'input>{
    pub fn new(node: &Node<'a, 'input>) -> Self {
        Self {
            node: node.clone()
        }
    }

    pub fn name(&self) -> &'a str {
        self.node.attribute(attribute::NAME).expect("Name required for wsdl:portType")
    }
}


