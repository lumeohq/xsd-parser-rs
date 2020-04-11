use crate::parser::constants::attribute;
use crate::parser::{ElementType, WsdlElement};
use roxmltree::Node;

// Content: Sequence [1..1]
// wsdl:documentation   [0..1] from type wsdl:tDocumented
// wsdl:operation       [0..*]

// Attributes:
// Any attribute   [0..*]		      Namespace: ##other, Process Contents: lax	from type wsdl:tExtensibleAttributesDocumented
// name	           [1..1] xsd:NCName

// Used in Group wsdl:anyTopLevelOptionalElement
// Type wsdl:tDefinitions via reference to wsdl:anyTopLevelOptionalElement (Element wsdl:definitions)

#[derive(Clone, Debug)]
pub struct PortType<'a> {
    node: Node<'a, 'a>,
    operations: Vec<Operation<'a>>,
}

impl<'a> PortType<'a> {
    pub fn new(node: &Node<'a, '_>) -> Self {
        Self {
            node: *node,
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

    pub fn name(&self) -> &'a str {
        self.node
            .attribute(attribute::NAME)
            .expect("Name required for wsdl:portType")
    }

    pub fn operations(&self) -> &[Operation] {
        self.operations.as_ref()
    }
}

// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tOperation
// Properties: Local, Qualified
//
// Content: Sequence [1..1]
// wsdl:documentation [0..1]   from type wsdl:tDocumented
// Any element [0..*] Namespace: ##other, Process Contents: lax   from type wsdl:tExtensibleDocumented
// Choice [1..1]
//     Sequence [1..1]   from group wsdl:request-response-or-one-way-operation
//         wsdl:input [1..1]
//         Sequence [0..1]
//             wsdl:output [1..1]
//             wsdl:fault [0..*]
//     Sequence [1..1]   from group wsdl:solicit-response-or-notification-operation
//         wsdl:output [1..1]
//         Sequence [0..1]
//             wsdl:input [1..1]
//             wsdl:fault [0..*]
//
// Attributes
// name	            [1..1]	xsd:NCName
// parameterOrder	[0..1]	xsd:NMTOKENS
//
// Used in
// Type wsdl:tPortType (Element wsdl:portType)
#[derive(Clone, Debug)]
pub struct Operation<'a> {
    node: Node<'a, 'a>,
    ty: OperationType<'a>,
}

impl<'a> Operation<'a> {
    pub fn new(node: &Node<'a, '_>) -> Self {
        Self {
            node: *node,
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

    pub fn operation_type(&self) -> &OperationType {
        &self.ty
    }

    pub fn documentation(&self) -> Option<&'a str> {
        self.node.children().find_map(|n| {
            if n.wsdl_type() == ElementType::Documentation {
                n.text()
            } else {
                None
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum OperationType<'a> {
    RequestResponse {
        input: Param<'a>,
        output: Param<'a>,
        faults: Vec<Fault<'a>>,
    },
    OneWay {
        input: Param<'a>,
    },
    SolicitResponse {
        output: Param<'a>,
        input: Param<'a>,
        faults: Vec<Fault<'a>>,
    },
    Notification {
        output: Param<'a>,
    },
}

impl<'a> OperationType<'a> {
    pub fn new(node: &Node<'a, '_>) -> Self {
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
                        OperationType::RequestResponse {
                            input: Param::new(&ch),
                            output: Param::new(&output_node),
                            faults: children
                                .filter_map(|n| match n.wsdl_type() {
                                    ElementType::Fault => Some(Fault::new(&n)),
                                    _ => None,
                                })
                                .collect(),
                        }
                    } else {
                        // OneWay
                        OperationType::OneWay {
                            input: Param::new(&ch),
                        }
                    };
                }
                ElementType::Output => {
                    // wsdl:solicit-response-or-notification-operation
                    return if let Some(input_node) = children.next() {
                        // SolicitResponse
                        assert_eq!(input_node.wsdl_type(), ElementType::Input);
                        OperationType::SolicitResponse {
                            output: Param::new(&ch),
                            input: Param::new(&input_node),
                            faults: children
                                .filter_map(|n| match n.wsdl_type() {
                                    ElementType::Fault => Some(Fault::new(&n)),
                                    _ => None,
                                })
                                .collect(),
                        }
                    } else {
                        OperationType::Notification {
                            output: Param::new(&ch),
                        }
                    };
                }
                _ => continue,
            };
        }
        unreachable!("Content of wsdl:operation must contain input or output")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Param<'a> {
    node: Node<'a, 'a>,
}

impl<'a> Param<'a> {
    pub fn new(node: &Node<'a, '_>) -> Self {
        Self { node: *node }
    }

    pub fn name(&self) -> Option<&'a str> {
        self.node.attribute(attribute::NAME)
    }

    pub fn message(&self) -> &'a str {
        self.node
            .attribute(attribute::MESSAGE)
            .expect("Message required for wsdl:input and wsdl:output")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fault<'a> {
    node: Node<'a, 'a>,
}

impl<'a> Fault<'a> {
    pub fn new(node: &Node<'a, '_>) -> Self {
        Self { node: *node }
    }

    pub fn name(&self) -> &'a str {
        self.node
            .attribute(attribute::NAME)
            .expect("Name required for wsdl:fault")
    }

    pub fn message(&self) -> &'a str {
        self.node
            .attribute(attribute::MESSAGE)
            .expect("Message required for wsdl:fault")
    }
}
