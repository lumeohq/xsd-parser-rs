// xsd:boolean
// The type xsd:boolean represents logical yes/no values. 
// The valid values for xsd:boolean are true, false, 0, and 1. 
// Values that are capitalized (e.g. TRUE) or abbreviated (e.g. T) are not valid.

// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema

// Schema Document: datatypes.xsd

// Content
// Based on xsd:anySimpleType
// White Space: collapse
// Examples
// Valid values	    Comment
// true	
// false	
// 0	            false
// 1	            true

// Invalid values	Comment
// TRUE	            values are case sensitive
// T	            the word "true" must be spelled out
//                  an empty value is not valid, unless xsi:nil is used
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:boolean
pub struct Boolean(pub bool);

impl Boolean {
    pub fn parse(s: &str) -> Result<Self, String> {
        if s == "0" || s == "false" {
            Ok(Self(false))
        } else if s == "1" || s == "true" {
            Ok(Self(true))
        } else {
            Err(format!("Invalid value for boolean: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Boolean;
    #[test]
    fn test_valid_values() {
        assert!(!Boolean::parse("0").unwrap().0);
        assert!(!Boolean::parse("0").unwrap().0);
        assert!(Boolean::parse("1").unwrap().0);
        assert!(Boolean::parse("true").unwrap().0);
    }

    #[test]
    fn test_invalid_values() {
        assert_eq!(Boolean::parse("2").err().unwrap(), "Invalid value for boolean: 2".to_string());
        assert!(Boolean::parse("True").is_err());
        assert!(Boolean::parse("FALSE").is_err());
        assert!(Boolean::parse("").is_err());
    }
}