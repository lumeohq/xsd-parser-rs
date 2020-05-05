use std::fmt;

// xsd:QName
// The type xsd:QName represents an XML namespace-qualified name. A xsd:QName value consists of a prefix and a local part, separated by a colon, both of which are NCName values. The prefix and colon are optional, but if they are not present, it is assumed that either the name is namespace-qualified by other means (e.g., by a default namespace declaration), or the name is not in a namespace.
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse
// Examples
// Valid values	        Comment
// pre:myElement	        valid assuming the prefix "pre" is mapped to a namespace in scope
// myElement	            prefix and colon are optional
// Invalid values	    Comment
// :myElement	            a QName must not start with a colon
// pre:3rdElement	        the local part must not start with a number; it must be a valid NCName
//                          an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:QName
#[derive(Default, Debug)]
pub struct QName<'a> {
    pub prefix: Option<&'a str>,
    pub name: &'a str,
}

impl<'a> QName<'a> {
    pub fn new(name: &'a str) -> Self {
        match name.find(':') {
            Some(index) => Self {
                prefix: Some(&name[0..index]),
                name: &name[index + 1..],
            },
            None => Self { prefix: None, name },
        }
    }
}

impl fmt::Display for QName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(prefix) = self.prefix {
            write!(f, "{}:{}", prefix, self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}
