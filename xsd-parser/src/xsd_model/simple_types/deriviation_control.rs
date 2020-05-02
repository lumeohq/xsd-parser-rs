// xsd:derivationControl
//    A utility type, not for public use
// Simple type information
// Namespace: http://www.w3.org/2001/XMLSchema

// Schema document: datatypes.xsd

// Content
// Type based on xsd:NMTOKEN
// Valid value
// substitution
// extension
// restriction
// list
// union

// Type inheritance chain
//  xsd:anySimpleType
//      xsd:string
//          xsd:normalizedString
//              xsd:token
//                  xsd:NMTOKEN
//                      xsd:derivationControl
//                          restricted by within xsd:simpleDerivationSet

#[derive(Debug, PartialEq)]
pub enum DeriviationControl {
    Substitution,
    Extension,
    Restriction,
    List,
    Union,
}

impl DeriviationControl {
    pub fn parse(s: &str) -> Result<Self, String> {
        Ok(match s {
            "substitution" => Self::Substitution,
            "extension" => Self::Extension,
            "restriction" => Self::Restriction,
            "list" => Self::List,
            "union" => Self::Union,
            _ => return Err(format!("Invalid value for DeriviationControl: {}", s)),
        })
    }
}

#[cfg(test)]
mod test {
    use super::DeriviationControl;
    #[test]
    fn test_parse() {
        assert_eq!(
            DeriviationControl::parse("substitution").unwrap(),
            DeriviationControl::Substitution
        );
        assert_eq!(
            DeriviationControl::parse("extension").unwrap(),
            DeriviationControl::Extension
        );
        assert_eq!(
            DeriviationControl::parse("restriction").unwrap(),
            DeriviationControl::Restriction
        );
        assert_eq!(
            DeriviationControl::parse("list").unwrap(),
            DeriviationControl::List
        );
        assert_eq!(
            DeriviationControl::parse("union").unwrap(),
            DeriviationControl::Union
        );
        assert!(DeriviationControl::parse("").is_err());
    }
}
