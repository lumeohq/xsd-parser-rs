// xsd:simpleDerivationSet
//    #all or (possibly empty) subset of {restriction, union, list}
//
//    A utility type, not for public use
// Simple type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: datatypes.xsd
//
// Content
//  Union of:
//      Type based on xsd:token
//          Valid value
//          #all
//      List of:
//          Type based on xsd:NMTOKEN
//              Valid value
//                  list
//                  union
//                  restriction
//
// Used by
// Attribute final
#[derive(Debug, PartialEq)]
pub enum SimpleDerivationSet {
    All,
    List(Vec<SimpleDerivationSubset>),
}

impl SimpleDerivationSet {
    pub fn parse(s: &str) -> Result<Self, String> {
        let res = match s {
            "#all" => Self::All,
            _ => {
                let res: Result<Vec<_>, String> = s
                    .split(' ')
                    .map(|s| SimpleDerivationSubset::parse(s))
                    .collect();
                Self::List(res?)
            }
        };
        Ok(res)
    }
}

#[derive(Debug, PartialEq)]
pub enum SimpleDerivationSubset {
    List,
    Union,
    Restriction,
}

impl SimpleDerivationSubset {
    pub fn parse(s: &str) -> Result<Self, String> {
        let res = match s {
            "list" => Self::List,
            "union" => Self::Union,
            "restriction" => Self::Restriction,
            _ => return Err(format!("Invalid value for SimpleDerivationSubset: {}", s)),
        };
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use super::SimpleDerivationSet;
    use super::SimpleDerivationSubset::*;

    #[test]
    fn test() {
        assert_eq!(
            SimpleDerivationSet::parse("#all").unwrap(),
            SimpleDerivationSet::All
        );
        assert_eq!(
            SimpleDerivationSet::parse("list union restriction").unwrap(),
            SimpleDerivationSet::List(vec![List, Union, Restriction])
        );
        assert!(SimpleDerivationSet::parse("val").is_err());
    }
}
