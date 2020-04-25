use xsd_types::types::negative_integer;

// xsd:negativeInteger
// The type xsd:negativeInteger represents an arbitrarily large negative integer. An xsd:negativeInteger is a sequence of digits, preceded by a - sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:nonPositiveInteger
// Maximum Inclusive: -1
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)
//
// Examples
// Valid values	    Comment
// -3
// -00122	          leading zeros are permitted
// Invalid values	Comment
// 0	              0 is not considered negative
// 122	              value cannot be positive
// +3	              value cannot be positive
// 3.0	              value must not contain a decimal point
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:decimal
//         restricted by xsd:integer
//             restricted by xsd:nonPositiveInteger
//                 restricted by xsd:negativeInteger

pub type NegativeInteger = negative_integer::NegativeInteger;
