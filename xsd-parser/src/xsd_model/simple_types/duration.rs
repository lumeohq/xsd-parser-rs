use xsd_types::types::duration;

// xsd:duration
// The type xsd:duration represents a duration of time expressed as a number of years, months, days, hours, minutes, and seconds. The format of xsd:duration is PnYnMnDTnHnMnS, where P is a literal value that starts the expression, nY is the number of years followed by a literal Y, nM is the number of months followed by a literal M, nD is the number of days followed by a literal D, T is a literal value that separates the date and time, nH is the number of hours followed by a literal H, nM is the number of minutes followed by a literal M, and nS is the number of seconds followed by a literal S. The following rules apply to xsd:duration values:
//
// Any of these numbers and corresponding designators may be absent if they are equal to 0, but at least one number and designator must appear.
//
// The numbers may be any unsigned integer, with the exception of the number of seconds, which may be an unsigned decimal number.
//
// If a decimal point appears in the number of seconds, there must be at least one digit after the decimal point.
//
// A minus sign may appear before the P to specify a negative duration.
//
// If no time items (hour, minute, second) are present, the letter T must not appear.
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
// Valid values         Comment
// P2Y6M5DT12H35M30S      2 years, 6 months, 5 days, 12 hours, 35 minutes, 30 seconds
// P1DT2H                 1 day, 2 hours
// P20M                   20 months (the number of months can be more than 12)
// PT20M                  20 minutes
// P0Y20M0D               20 months (0 is permitted as a number, but is not required)
// P0Y                    0 years
// -P60D                  minus 60 days
// PT1M30.5S              1 minute, 30.5 seconds
// Invalid values       Comment
// P-20M                  the minus sign must appear first
// P20MT                  no time items are present, so "T" must not be present
// P1YM5D                 no value is specified for months, so "M" must not be present
// P15.5Y                 only the seconds can be expressed as a decimal
// P1D2H                  "T" must be present to separate days and hours
// 1Y2M                   "P" must always be present
// P2M1Y                  years must appear before months
// P                      at least one number and designator are required
// PT15.S                 at least one digit must follow the decimal point if it appears
//                        an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
// xsd:anySimpleType
//     restricted by xsd:duration
//         restricted by xsd:dayTimeDuration
//         restricted by xsd:yearMonthDuration

pub type Date = duration::Duration;
