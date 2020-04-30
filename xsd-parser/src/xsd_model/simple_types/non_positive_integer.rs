use xsd_types::types::non_positive_integer;

// xsd:nonPositiveInteger
// The type xsd:nonPositiveInteger represents an arbitrarily large non-positive integer. An xsd:nonPositiveInteger is a sequence of digits, optionally preceded by a - sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:integer
// Fraction Digits: 0 (Defined in type xsd:integer)
// Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
// White Space: collapse (Defined in type xsd:decimal)
// Maximum Inclusive: 0
//
// Examples
// Valid values     Comment
// -3
// 0
// -00122             leading zeros are permitted
// Invalid values   Comment
// 122                value cannot be positive
// +3                 value cannot be positive
// 3.0                value must not contain a decimal point
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
//     xsd:anySimpleType
//         restricted by xsd:decimal
//             restricted by xsd:integer
//                 restricted by xsd:nonPositiveInteger
//                     restricted by xsd:negativeInteger

pub type NonPositiveInteger = non_positive_integer::NonPositiveInteger;
