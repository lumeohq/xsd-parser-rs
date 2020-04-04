use roxmltree::Node;
use crate::parser::constants::attribute;
use crate::parser::{WsdlElement, ElementType};

// Element information:
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tMessage
// Properties: Local, Qualified

// Content: Sequence [1..1]
// wsdl:documentation [0..1]   from type wsdl:tDocumented
// Any element [0..*] Namespace: ##other, Process Contents: lax   from type wsdl:tExtensibleDocumented
// wsdl:part [0..*]
//
// Attributes:
// name	[1..1]	xsd:NCName

// Identity constraints:
// Type	    Name	Selector	Field(s)
// unique	part	wsdl:part	@name
//
// Used in
// Group wsdl:anyTopLevelOptionalElement
// Type wsdl:tDefinitions via reference to wsdl:anyTopLevelOptionalElement (Element wsdl:definitions)
#[derive(Clone, Debug)]
pub struct Message<'a> {
    node: Node<'a, 'a>,
    parts: Vec<Part<'a>>
}

impl<'a> Message<'a> {
    pub fn new(node: &Node<'a, '_>) -> Self {
        Self {
            node: node.clone(),
            parts: node.children().filter_map(|n| if n.is_element() && n.wsdl_type() == ElementType::Part {Some(Part::new(&n))}else {None}).collect()
        }
    }

    pub fn name(&self) -> &'a str {
        self.node
            .attribute(attribute::NAME)
            .expect("Name required for wsdl:message")
    }

    pub fn parts(&self) -> &[Part<'a>] {
        self.parts.as_ref()
    }
}



// Element information
// Namespace: http://schemas.xmlsoap.org/wsdl/
// Schema document: wsdl11.xsd
// Type: wsdl:tPart
// Properties: Local, Qualified
//
// Content:
// wsdl:documentation [0..1]   from type wsdl:tDocumented
//
// Attributes:
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type wsdl:tExtensibleAttributesDocumented
// name	            [1..1]	xsd:NCName
// element	        [0..1]	xsd:QName
// type	            [0..1]	xsd:QName
//
// Used in
// Type wsdl:tMessage (Element wsdl:message)
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