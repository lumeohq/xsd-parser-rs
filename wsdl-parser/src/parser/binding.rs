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
    input: Option<Input<'a, 'input>>,
    output: Option<Output<'a, 'input>>,
    faults: Vec<Fault<'a, 'input>>,
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
                ElementType::Input => {input = Some(Input::new(&ch))}
                ElementType::Output => {output = Some(Output::new(&ch))}
                ElementType::Fault => {faults.push(Fault::new(&ch))}
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


#[derive(Clone, Debug, PartialEq)]
pub struct Input<'a, 'input: 'a> {
    node: Node<'a, 'input>,
}

impl<'a, 'input: 'a> Input<'a, 'input> {
    pub fn new(node: &Node<'a, 'input>) -> Self {
        Self { node: node.clone() }
    }

    pub fn name(&self) -> Option<&'a str> {
        self.node.attribute(attribute::NAME)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Output<'a, 'input: 'a> {
    node: Node<'a, 'input>,
}

impl<'a, 'input: 'a> Output<'a, 'input> {
    pub fn new(node: &Node<'a, 'input>) -> Self {
        Self { node: node.clone() }
    }

    pub fn name(&self) -> Option<&'a str> {
        self.node.attribute(attribute::NAME)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fault<'a, 'input: 'a> {
    node: Node<'a, 'input>,
}

impl<'a, 'input: 'a> Fault<'a, 'input> {
    pub fn new(node: &Node<'a, 'input>) -> Self {
        Self { node: node.clone() }
    }

    pub fn name(&self) -> &'a str {
        self.node.attribute(attribute::NAME).expect("Name required for wsdl:fault")
    }
}
