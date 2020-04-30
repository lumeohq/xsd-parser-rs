use xsd_types::types::gyearmonth;

// xsd:gYearMonth
// The type xsd:gYearMonth represents a specific month of a specific year. The letter g signifies "Gregorian." The format of xsd:gYearMonth is CCYY-MM. No left truncation is allowed on either part. To represents years later than 9999, additional digits can be added to the left of the year value. To represent years before 0001, a preceding minus sign ("-") is permitted.
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
//
// Examples
// Valid values     Comment
// 2004-04            April 2004
// 2004-04-05:00      April 2004, US Eastern Standard Time
// Invalid values   Comment
// 99-04              the century must not be truncated
// 2004               the month is required
// 2004-4             the month must be two digits
// 2004-13            the month must be a valid month
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:gYearMonth

pub type GYearMonth = gyearmonth::GYearMonth;
