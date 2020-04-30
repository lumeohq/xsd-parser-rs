use xsd_types::types::gmonthday;

// xsd:gMonthDay
// The type xsd:gMonthDay represents a specific day that recurs every year. The letter g signifies "Gregorian." xsd:gMonthDay can be used to say, for example, that your birthday is on the 14th of April every year. The format of xsd:gMonthDay is --MM-DD.
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
// --04-12            April 12
// --04-12Z           April 12, Coordinated Universal Time (UTC)
// Invalid values   Comment
// 04-12              the leading hyphens are required
// --04-31            it must be a valid day of the year (April has 30 days)
// --4-6              the month and day must be 2 digits each
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:gMonthDay

pub type GMonthDay = gmonthday::GMonthDay;
