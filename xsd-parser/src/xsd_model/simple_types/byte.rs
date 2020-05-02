// xsd:byte
// The type xsd:byte represents an integer between -128 and 127.
// An xsd:byte is a sequence of digits, optionally preceded by a + or - sign.
// Leading zeros are permitted, but decimal points are not.

// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema

// Schema Document: datatypes.xsd

// Content
// Based on xsd:short
// Minimum Inclusive: -128
// Maximum Inclusive: 127
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)

// Examples
// Valid values	Comment
// +3
// 122
// 0
// -123

// Invalid values	Comment
// 130	            number is too large
// 3.0	            value must not contain a decimal point
//                  an empty value is not valid, unless xsi:nil is used

// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:decimal
//          restricted by xsd:integer
//              restricted by xsd:long
//                  restricted by xsd:int
//                      restricted by xsd:short
//                          restricted by xsd:byte

pub type Byte = i8;

#[cfg(test)]
mod tests {
    use super::Byte;

    #[test]
    fn test_valid_values() {
        assert_eq!("+3".parse::<Byte>().unwrap(), 3);
        assert_eq!("122".parse::<Byte>().unwrap(), 122);
        assert_eq!("0".parse::<Byte>().unwrap(), 0);
        assert_eq!("-123".parse::<Byte>().unwrap(), -123);
    }

    #[test]
    fn test_invalid_values() {
        assert!("130".parse::<Byte>().is_err());
        assert!("3.0".parse::<Byte>().is_err());
        assert!("".parse::<Byte>().is_err());
    }
}
