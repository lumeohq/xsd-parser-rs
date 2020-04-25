use xsd_types::types::gmonth;

// xsd:gMonth
// The type xsd:gMonth represents a specific month that recurs every year. The letter g signifies "Gregorian." xsd:gMonth can be used to indicate, for example, that fiscal year-end processing occurs in September of every year. To represent a duration of months, use the duration type instead. The format of xsd:gMonth is --MM.
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
// --04	April
// --04-05:00         April, US Eastern Standard Time
// Invalid values   Comment
// 2004-04            the year must not be specified; use gYearMonth instead
// 04                 the leading hyphens are required
// --4                the month must be 2 digits
// --13               the month must be a valid month
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:gMonth

pub type GMonth = gmonth::GMonth;
