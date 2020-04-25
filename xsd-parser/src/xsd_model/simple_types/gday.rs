use xsd_types::types::gday;

// xsd:gDay
// The type xsd:gDay represents a day that recurs every month. The letter g signifies "Gregorian." xsd:gDay can be used to say, for example, that checks are paid on the 5th of each month. To represent a duration of days, use the duration type instead. The format of gDay is ---DD.
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
// ---02              the 2nd of the month
// Invalid values   Comment
// 02                 the leading hyphens are required
// ---2               the day must be 2 digits
// ---32              the day must be a valid day of the month; no month has 32 days
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:gDay

pub type GDay = gday::GDay;
