use crate::parser::constants::attribute;
use crate::parser::{ElementType, WsdlElement};
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct Binding<'a, 'input: 'a> {
    node: Node<'a, 'input>,
    operations: Vec<Operation<'a, 'input>>,
}

impl<'a, 'input: 'a> Binding<'a, 'input> {
    pub fn name(&self) -> &'a str {
        self.node
            .attribute(attribute::NAME)
            .expect("Namespace required for wsdl:binding")
    }

    pub fn type_(&self) -> &'a str {
        self.node
            .attribute(attribute::TYPE)
            .expect("Location required for wsdl:binding")
    }

    pub fn new(node: &Node<'a, 'input>) -> Self {
        Self {
            node: node.clone(),
            operations: node
                .children()
                .filter_map(|node| {
                    if node.is_element() && node.wsdl_type() == ElementType::Operation {
                        Some(Operation::new(&node))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Operation<'a, 'input: 'a> {
    node: Node<'a, 'input>,
    input: Option<Param<'a, 'input>>,
    output: Option<Param<'a, 'input>>,
    faults: Vec<Param<'a, 'input>>,
}

impl<'a, 'input: 'a> Operation<'a, 'input> {
    pub fn name(&self) -> &'a str {
        self.node
            .attribute(attribute::NAME)
            .expect("Namespace required for wsdl:binding")
    }

    pub fn new(node: &Node<'a, 'input>) -> Self {
        let mut input = None;
        let mut output = None;
        let mut faults = vec![];
        for ch in node.children().filter(|n| n.is_element()) {
            match ch.wsdl_type() {
                ElementType::Input => {input = Some(Param::new(&ch))}
                ElementType::Output => {output = Some(Param::new(&ch))}
                ElementType::Fault => {faults.push(Param::new(&ch))}
                _ => {}
            }
        }
        Self {
            node: node.clone(),
            input,
            output,
            faults,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Param<'a, 'input: 'a> {
    node: Node<'a, 'input>,
}

impl<'a, 'input: 'a> Param<'a, 'input> {
    pub fn new(node: &Node<'a, 'input>) -> Self {
        Self { node: node.clone() }
    }

    pub fn name(&self) -> Option<&'a str> {
        self.node.attribute(attribute::NAME)
    }
}
