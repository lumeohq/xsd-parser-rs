use crate::parser::binding::Binding;
use crate::parser::constants::attribute;
use crate::parser::message::Message;
use crate::parser::port_type::PortType;
use crate::parser::types::Types;
use crate::parser::{ElementType, WsdlElement};
use roxmltree::{Document, Namespace, Node};
use std::collections::HashMap;

// Content: Sequence [1..1]
//      wsdl:documentation [0..1]  from type wsdl:tDocumented
//      Any element [0..*] Namespace: ##other, Process Contents: lax  from type wsdl:tExtensibleDocumented
//      Choice [0..*]   from group wsdl:anyTopLevelOptionalElement
//          wsdl:import
//          wsdl:types
//          wsdl:message
//          wsdl:portType
//          wsdl:binding
//          wsdl:service

// Attributes:
// targetNamespace	[0..1]	xsd:anyURI
// name	            [0..1]	xsd:NCName

// Identity constraints:
// Type	    Name	    Selector	    Field(s)
// key	    message	    wsdl:message	@name
// key	    portType    wsdl:portType	@name
// key	    binding	    wsdl:binding	@name
// key	    service	    wsdl:service	@name
// key	    import	    wsdl:import	    @namespace

#[derive(Debug)]
pub struct Definitions<'a> {
    node: Node<'a, 'a>,
    imports: Vec<Import<'a>>,
    types: Vec<Types<'a>>,
    messages: HashMap<&'a str, Message<'a>>,
    port_types: HashMap<&'a str, PortType<'a>>,
    bindings: HashMap<&'a str, Binding<'a>>,
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

    pub fn types(&self) -> &[Types] {
        self.types.as_ref()
    }

    pub fn messages(&self) -> &HashMap<&'a str, Message<'a>> {
        &self.messages
    }

    pub fn new(definitions: &Node<'a, '_>) -> Self {
        let mut res = Self {
            node: definitions.clone(),
            imports: vec![],
            messages: HashMap::new(),
            port_types: HashMap::new(),
            types: vec![],
            bindings: HashMap::new(),
        };
        for ch in definitions.children().filter(|n| n.is_element()) {
            match ch.wsdl_type() {
                ElementType::Import => res.imports.push(Import::new(&ch)),
                ElementType::Types => res.types.push(Types::new(&ch)), // TODO: add Identity constraints: @namespace
                ElementType::Message => res.add_message(&ch),
                ElementType::PortType => res.add_port_type(&ch),
                ElementType::Binding => res.add_binding(&ch),
                _ => {}
            }
        }
        res
    }

    fn add_message(&mut self, node: &Node<'a, '_>) {
        let message = Message::new(node);
        assert!(
            self.messages.insert(message.name(), message).is_none(),
            "Message name must be unique"
        );
    }

    fn add_port_type(&mut self, node: &Node<'a, '_>) {
        let port_type = PortType::new(node);
        assert!(
            self.port_types
                .insert(port_type.name(), port_type)
                .is_none(),
            "portType name must be unique"
        );
    }

    fn add_binding(&mut self, node: &Node<'a, '_>) {
        let binding = Binding::new(node);
        assert!(
            self.bindings.insert(binding.name(), binding).is_none(),
            "binding name must be unique"
        );
    }
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
