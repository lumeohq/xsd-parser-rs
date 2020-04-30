use xsd_types::types::time;

// xsd:time
// The type xsd:time represents a time of day in the format hh:mm:ss.sss where hh represents the hour, mm the minutes, and ss.sss the seconds. An unlimited number of additional digits can be used to increase the precision of fractional seconds if desired. The time is based on a 24-hour time period, so hours should be represented as 00 through 24. Either of the values 00:00:00 or 24:00:00 can be used to represent midnight.
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
// Valid values       Comment
// 13:20:00	1:20 PM
// 13:20:30.5555        1:20 PM and 30.5555 seconds
// 13:20:00-05:00       1:20 PM, US Eastern Standard Time
// 13:20:00Z            1:20 PM, Coordinated Universal Time (UTC)
// 00:00:00             midnight
// 24:00:00             midnight
// Invalid values     Comment
// 5:20:00              hours, minutes, and seconds must be two digits each
// 13:20                seconds must be specified, even if it is 00
// 13:20.5:00           values for hours and minutes must be integers
// 13:65:00             the value must be a valid time of day
//                      an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:time

pub type Time = time::Time;
