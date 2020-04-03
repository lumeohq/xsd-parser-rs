use crate::parser::constants::attribute;
use crate::parser::{ElementType, WsdlElement};
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct Operation<'a, 'input: 'a> {
    node: Node<'a, 'input>,
    ty: OperationType<'a, 'input>,
}

impl<'a, 'input: 'a> Operation<'a, 'input> {
    pub fn new(node: &Node<'a, 'input>) -> Self {
        Self {
            node: node.clone(),
            ty: OperationType::new(node),
        }
    }

    pub fn name(&self) -> &'a str {
        self.node
            .attribute(attribute::NAME)
            .expect("Name required for wsdl:operation")
    }

    pub fn parameter_order(&self) -> Option<&'a str> {
        self.node.attribute(attribute::PARAMETER_ORDER)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum OperationType<'a, 'input> {
    RequestResponse(
        Input<'a, 'input>,
        Output<'a, 'input>,
        Vec<Fault<'a, 'input>>,
    ),
    OneWay(Input<'a, 'input>),
    SolicitResponse(
        Output<'a, 'input>,
        Input<'a, 'input>,
        Vec<Fault<'a, 'input>>,
    ),
    Notification(Output<'a, 'input>),
}

impl<'a, 'input: 'a> OperationType<'a, 'input> {
    pub fn new(node: &Node<'a, 'input>) -> Self {
        let mut children = node.children().filter(|n| n.is_element());

        while let Some(ch) = children.next() {
            match ch.wsdl_type() {
                ElementType::Input => {
                    // wsdl:request-response-or-one-way-operation
                    return if let Some(output_node) = children.next() {
                        // RequestResponse
                        assert_eq!(
                            output_node.wsdl_type(),
                            ElementType::Output,
                            "{}",
                            format!("{:?}", output_node)
                        );
                        OperationType::RequestResponse(
                            Input::new(&ch),
                            Output::new(&output_node),
                            children
                                .filter_map(|n| match n.wsdl_type() {
                                    ElementType::Fault => Some(Fault::new(&n)),
                                    _ => None,
                                })
                                .collect(),
                        )
                    } else {
                        // OneWay
                        OperationType::OneWay(Input::new(&ch))
                    };
                }
                ElementType::Output => {
                    // wsdl:solicit-response-or-notification-operation
                    return if let Some(input_node) = children.next() {
                        // SolicitResponse
                        assert_eq!(input_node.wsdl_type(), ElementType::Input);
                        OperationType::SolicitResponse(
                            Output::new(&ch),
                            Input::new(&input_node),
                            children
                                .filter_map(|n| match n.wsdl_type() {
                                    ElementType::Fault => Some(Fault::new(&n)),
                                    _ => None,
                                })
                                .collect(),
                        )
                    } else {
                        OperationType::Notification(Output::new(&ch))
                    };
                }
                _ => continue,
            };
        }
        unreachable!("Content of wsdl:operation must contain input or output")
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

    pub fn message(&self) -> &'a str {
        self.node
            .attribute(attribute::MESSAGE)
            .expect("Message required for wsdl:input")
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

    pub fn message(&self) -> &'a str {
        self.node
            .attribute(attribute::MESSAGE)
            .expect("Message required for wsdl:input")
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

    pub fn name(&self) -> Option<&'a str> {
        self.node.attribute(attribute::NAME)
    }

    pub fn message(&self) -> &'a str {
        self.node
            .attribute(attribute::MESSAGE)
            .expect("Message required for wsdl:fault")
    }
}
