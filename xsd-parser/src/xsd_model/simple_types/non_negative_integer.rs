use std::str::FromStr;

// xsd:nonNegativeInteger
// The type xsd:nonNegativeInteger represents an arbitrarily large non-negative integer. An xsd:nonNegativeInteger is a sequence of digits, optionally preceded by a + sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:integer
// Minimum Inclusive: 0
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)
//
// Examples
// Valid values	      Comment
// +3
// 122
// 0
// 00122	            leading zeros are permitted
// Invalid values	 Comment
// -3	                value cannot be negative
// 3.0	                value must not contain a decimal point
//                      an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//  restricted by xsd:decimal
//      restricted by xsd:integer
//          restricted by xsd:nonNegativeInteger
//              restricted by xsd:unsignedLong
//                  restricted by xsd:unsignedInt
//                      restricted by xsd:unsignedShort
//                          restricted by xsd:unsignedByte
//          restricted by xsd:positiveInteger
#[derive(Default, Debug, PartialEq)]
pub struct NonNegativeInteger(pub usize);

impl NonNegativeInteger{
    pub fn parse(s: &str) -> Result<Self, String> {
        let res = s
            .parse::<usize>()
            .map_err(|er| format!("parse xsd:nonNegativeInteger error: {}", er.to_string()))?;
        Ok(Self(res))
    }
}

impl FromStr for NonNegativeInteger {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[test]
fn test_parse() {
    assert_eq!("1".parse::<NonNegativeInteger>().unwrap().0, 1);
    assert_eq!("+10".parse::<NonNegativeInteger>().unwrap().0, 10);
    assert_eq!("000122".parse::<NonNegativeInteger>().unwrap().0, 122);
    assert_eq!("0".parse::<NonNegativeInteger>().unwrap().0, 0);

    assert_eq!(
        "-3".parse::<NonNegativeInteger>().err().unwrap(),
        "parse xsd:positiveInteger error: invalid digit found in string"
    );
    assert_eq!(
        "3.0".parse::<NonNegativeInteger>().err().unwrap(),
        "parse xsd:positiveInteger error: invalid digit found in string"
    );
}
