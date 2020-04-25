use xsd_types::types::gyear;

// xsd:gYear
// The type xsd:gYear represents a specific calendar year. The letter g signifies "Gregorian." The format of xsd:gYear is CCYY. No left truncation is allowed. To represent years later than 9999, additional digits can be added to the left of the year value. To represent years before 0001, a preceding minus sign ("-") is allowed.
//
// An optional time zone expression may be added at the end of the value. The letter Z is used to indicate Coordinated Universal Time (UTC). All other time zones are represented by their difference from Coordinated Universal Time in the format +hh:mm, or -hh:mm. These values may range from -14:00 to 14:00. For example, US Eastern Standard Time, which is five hours behind UTC, is represented as -05:00. If no time zone value is present, it is considered unknown; it is not assumed to be UTC.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:anySimpleType
// White Space: collapse
// Examples
// Valid values     Comment
// 2004	2004
// 2004-05:00         2004, US Eastern Standard Time
// 12004              the year 12004
// 0922               the year 922
// -0045              45 BC
// Invalid values   Comment
// 99                  the century must not be truncated
// 922                 no left truncation is allowed; leading zeros should be added if necessary
//                     an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:gYear

pub type GYear = gyear::GYear;
