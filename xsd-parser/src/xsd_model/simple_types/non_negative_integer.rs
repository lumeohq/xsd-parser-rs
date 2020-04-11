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

impl NonNegativeInteger {
    pub fn new(s: &str) -> Self {
        Self {
            0: s.parse::<usize>().expect("Value mus be a digit")  //TODO: maybe fromStr
        }
    }
}


#[test]
fn test_valid_values() {
}

#[test]
fn test_invalid_values() {
}