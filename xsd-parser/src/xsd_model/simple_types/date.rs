use xsd_types::types::date;

// xsd:date
// The type xsd:date represents a Gregorian calendar date in the format CCYY-MM-DD where CC represents the century, YY the year, MM the month and DD the day. No left truncation is allowed for any part of the date. To represent years later than 9999, additional digits can be added to the left of the year value, but extra leading zeros are not permitted. To represent years before 0001, a preceding minus sign ("-") is allowed. The year 0000 is not a valid year in the Gregorian calendar.
//
// An optional time zone expression may be added at the end. The letter Z is used to indicate Coordinated Universal Time (UTC). All other time zones are represented by their difference from Coordinated Universal Time in the format +hh:mm, or -hh:mm. These values may range from -14:00 to 14:00. For example, US Eastern Standard Time, which is five hours behind UTC, is represented as -05:00. If no time zone value is present, it is considered unknown; it is not assumed to be UTC.
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
// Valid values	    Comment
// 2004-04-12         April 12, 2004
// -0045-01-01        January 1, 45 BC
// 12004-04-12        April 12, 12004
// 2004-04-12-05:00	  April 12, 2004, US Eastern Standard Time, which is 5 hours behind Coordinated Universal Time (UTC)
// 2004-04-12Z	      April 12, 2004, Coordinated Universal Time (UTC)
// Invalid values   Comment
// 99-04-12           left truncation of the century is not allowed
// 2004-4-2           month and day must be two digits each
// 2004/04/02         slashes are not valid separators
// 04-12-2004         the value must be in CCYY-MM-DD order
// 2004-04-31         the date must be a valid date (April has 30 days)
//                    an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:date

pub type Date = date::Date;
