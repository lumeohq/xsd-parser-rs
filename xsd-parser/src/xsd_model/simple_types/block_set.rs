// xsd:blockSet
// #all or (possibly empty) subset of {substitution, extension, restriction}
// Simple type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Union of:
//      Type based on xsd:token
//          Valid value
//          #all
//      List of:
//          Type based on xsd:NMTOKEN
//              Valid value
//              extension
//              restriction
//              substitution
//
// Used by
// Attribute block
// Attribute blockDefault
#[derive(Debug, PartialEq)]
pub enum BlockSet {
    All,
    List(Vec<BlockSetChoice>),
}

#[derive(Debug, PartialEq)]
pub enum BlockSetChoice {
    Extension,
    Restriction,
    Substitution,
}

impl BlockSetChoice {
    pub fn parse(s: &str) -> Result<Self, String> {
        let res = match s {
            "extension" => Self::Extension,
            "restriction" => Self::Restriction,
            "substitution" => Self::Substitution,
            _ => return Err(format!("Invalid value for BlockSet: {}", s)),
        };
        Ok(res)
    }
}

impl BlockSet {
    pub fn parse(s: &str) -> Result<Self, String> {
        let res = if s == "#all" {
            Self::All
        } else {
            let res: Result<Vec<_>, String> =
                s.split(' ').map(|v| BlockSetChoice::parse(v)).collect();
            Self::List(res?)
        };
        Ok(res)
    }
}
