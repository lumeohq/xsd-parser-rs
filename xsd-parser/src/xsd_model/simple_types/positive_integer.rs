// xsd:positiveInteger
// The type xsd:positiveInteger represents an arbitrarily large positive integer. An xsd:positiveInteger is a sequence of digits, optionally preceded by a + sign. Leading zeros are permitted, but decimal points are not.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
//  Based on xsd:nonNegativeInteger
//  Minimum Inclusive: 1
//  Fraction Digits: 0 (Defined in type xsd:integer)
//  Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
//  White Space: collapse (Defined in type xsd:decimal)
//
// Examples
// Valid values	    Comment
// 122
// +3
// 00122	        leading zeros are permitted
// Invalid values	Comment
// 0	            0 is not considered positive
// -3	            value cannot be negative
// 3.0	            value must not contain a decimal point
//                  an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
// restricted by xsd:decimal
// restricted by xsd:integer
// restricted by xsd:nonNegativeInteger
// restricted by xsd:positiveInteger
pub struct PositiveInteger(usize);