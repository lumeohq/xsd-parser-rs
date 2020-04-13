// xsd:fullDerivationSet
// #all or (possibly empty) subset of {extension, restriction, list, union}
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
//              Valid value	Description
//              extension	    Extension is disallowed
//              restriction	    Restriction is disallowed
//              list	        Derivation by list is disallowed
//              union	        Derivation by union is disallowed
//
// Used by
// Attribute finalDefault
#[derive(Debug, PartialEq)]
pub enum FullDerivationSet {
    All,
    List(Vec<FullDerivationSubSet>),
}

#[derive(Debug, PartialEq)]
pub enum FullDerivationSubSet {
    Extension,
    Restriction,
    List,
    Union,
}

impl FullDerivationSubSet {
    pub fn parse(s: &str) -> Result<Self, String> {
        let res = match s {
            "extension" => Self::Extension,
            "restriction" => Self::Restriction,
            "list" => Self::List,
            "union" => Self::Union,
            _ => return Err(format!("Invalid value for FullDerivationSet: {}", s)),
        };
        Ok(res)
    }
}

impl FullDerivationSet {
    pub fn parse(s: &str) -> Result<Self, String> {
        let res = if s == "#all" {
            Self::All
        } else {
            let res: Result<Vec<_>, String> = s
                .split(' ')
                .map(|v| FullDerivationSubSet::parse(v))
                .collect();
            Self::List(res?)
        };
        Ok(res)
    }
}
