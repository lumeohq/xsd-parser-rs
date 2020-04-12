// xsd:formChoice
// Whether local declarations are qualified (in a namespace), or unqualified (in no namespace).  This setting does not affect global declarations.
// Simple type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
// Type based on xsd:NMTOKEN
//      Valid value	    Description
//      qualified	    Local declarations are qualified (in a namespace)
//      unqualified	    Local declarations are unqualified (not in a namespace)
//
// Used by
// Attribute attributeFormDefault
// Attribute elementFormDefault
// Attribute form
//
// Type inheritance chain
//  xsd:anySimpleType
//      xsd:string
//          xsd:normalizedString
//              xsd:token
//                  xsd:NMTOKEN
//                      xsd:formChoice
#[derive(Debug, PartialEq)]
pub enum FormChoice{
    Qualified,
    Unqualified
}

impl Default for FormChoice {
    fn default() -> Self {
        FormChoice::Unqualified
    }
}

impl FormChoice {
    pub fn parse(s: &str) -> Result<Self, String> {
        let res = match s {
            "qualified" => Self::Qualified,
            "unqualified" => Self::Unqualified,
            _ => Err(format!("Invalid value for FormChoice: {}", s))?
        };
        Ok(res)
    }
}