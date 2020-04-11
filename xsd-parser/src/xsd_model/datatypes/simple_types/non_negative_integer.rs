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
pub struct NonNegativeInteger(usize);
//TODO: fromStr with validation

impl FromStr for NonNegativeInteger {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self{ 0: s.parse::<usize>().map_err(|err| "Invalid ")?})
    }
}


#[test]
fn test_valid_values() {
    assert_eq!("+3".parse::<NonNegativeInteger>().0, 3usize);
    assert_eq!("122".parse::<NonNegativeInteger>().0, 122usize);
    assert_eq!("0".parse::<NonNegativeInteger>().0, 0usize);
    assert_eq!("0".parse::<NonNegativeInteger>().0, 0usize);
    assert_eq!("00122".parse::<NonNegativeInteger>().0, 122usize);
}

#[test]
fn test_invalid_values() {
    assert_eq!("-3".parse::<NonNegativeInteger>(), Err("Value cannot be negative"));
    assert_eq!("3.0".parse::<NonNegativeInteger>(), Err("Value must not contain a decimal point"));
    assert_eq!("".parse::<NonNegativeInteger>(), Err("An empty value is not valid"));
}