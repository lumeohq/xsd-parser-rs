use xsd_types::types::datetime;

// xsd:dateTime
// The type xsd:dateTime represents a specific date and time in the format CCYY-MM-DDThh:mm:ss.sss, which is a concatenation of the date and time forms, separated by a literal letter "T". All of the same rules that apply to the date and time types are applicable to xsd:dateTime as well.
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
// Valid values              Comment
// 2004-04-12T13:20:00         1:20 pm on April 12, 2004
// 2004-04-12T13:20:15.5       1:20 pm and 15.5 seconds on April 12, 2004
// 2004-04-12T13:20:00-05:00   1:20 pm on April 12, 2004, US Eastern Standard Time
// 2004-04-12T13:20:00Z	1:20   pm on April 12, 2004, Coordinated Universal Time (UTC)
// Invalid values            Comment
// 2004-04-12T13:00            seconds must be specified
// 2004-04-1213:20:00          the letter T is required
// 99-04-12T13:00              the century must not be left truncated
// 2004-04-12                  the time is required
//                             an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:dateTime

pub type DateTime = datetime::DateTime;
