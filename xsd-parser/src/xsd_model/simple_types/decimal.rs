use xsd_types::types::decimal;

// xsd:decimal
// The type xsd:decimal represents a decimal number of arbitrary precision. Schema processors vary in the number of significant digits they support, but a conforming processor must support a minimum of 18 significant digits. The format of xsd:decimal is a sequence of digits optionally preceded by a sign ("+" or "-") and optionally containing a period. The value may start or end with a period. If the fractional part is 0 then the period and trailing zeros may be omitted. Leading and trailing zeros are permitted, but they are not considered significant. That is, the decimal values 3.0 and 3.0000 are considered equal.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse
//
// Examples
// Valid values     Comment
// 3.0
// -3.0               a negative sign is permitted
// +3.5               a positive sign is permitted
// 3                  a decimal point is not required
// .3                 the value can start with a decimal point
// 3.                 the value can end with a decimal point
// 0
// -.3
// 0003.0             leading zeros are permitted
// 3.0000             trailing zeros are permitted
// Invalid values   Comment
// 3,5                commas are not permitted; the decimal separator must be a period
//                    an empty value is not valid, unless xsi:nil is used
//
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

pub type Date = decimal::Decimal;
