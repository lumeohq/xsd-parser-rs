use xsd_types::types::integer;

// xsd:integer
// The type xsd:integer represents an arbitrarily large integer, from which twelve other built-in integer types are derived (directly or indirectly). An xsd:integer is a sequence of digits, optionally preceded by a + or - sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:decimal
// Fraction Digits: 0
// Pattern: [\-+]?[0-9]+
// White Space: collapse (Defined in type xsd:decimal)

// Examples
// Valid values     Comment
// 122
// 00122	          leading zeros are permitted
// 0
// -3
// +3
// Invalid values	Comment
// 3.	              an integer must not contain a decimal point
// 3.0	              an integer must not contain a decimal point
//                    an empty value is not valid, unless xsi:nil is used

// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:decimal
//         restricted by xsd:integer
//             restricted by xsd:nonPositiveInteger
//                 restricted by xsd:negativeInteger
//             restricted by xsd:long
//                 restricted by xsd:int
//                     restricted by xsd:short
//                         restricted by xsd:byte
//             restricted by xsd:nonNegativeInteger
//                 restricted by xsd:unsignedLong
//                     restricted by xsd:unsignedInt
//                         restricted by xsd:unsignedShort
//                             restricted by xsd:unsignedByte
//                 restricted by xsd:positiveInteger

pub type Integer = integer::Integer;
