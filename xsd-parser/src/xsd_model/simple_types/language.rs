// xsd:language
// The type xsd:language represents a natural language identifier, generally used to indicate the language of a document or a part of a document. Before creating a new attribute of type xsd:language, consider using the xml:lang attribute that is intended to indicate the natural language of the element and its content.
// Values of the xsd:language type conform to RFC 3066, Tags for the Identification of Languages. The three most common formats are:
// For ISO-recognized languages, the format is a two- or three-letter, (usually lowercase) language code that conforms to ISO 639, optionally followed by a hyphen and a two-letter, (usually uppercase) country code that conforms to ISO 3166. For example, en or en-US.
// For languages registered by the Internet Assigned Numbers Authority (IANA), the format is i-langname, where langname is the registered name. For example, i-navajo.
// For unofficial languages, the format is x-langname, where langname is a name of up to eight characters agreed upon by the two parties sharing the document. For example, x-Newspeak.
// Any of these three formats may have additional parts, each preceded by a hyphen, which identify additional countries or dialects. Schema processors will not verify that values of the xsd:language type conform to the above rules. They will simply validate based on the pattern specified for this type.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:token
// Pattern: [a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*
// White Space: collapse (Defined in type xsd:token)
//
// Examples
// Valid values	                Comment
// en	                        English
// en-GB	                    UK English
// en-US	                    US English
// fr	                        French
// de	                        German
// es	                        Spanish
// it	                        Italian
// nl	                        Dutch
// zh	                        Chinese
// ja	                        Japanese
// ko	                        Korean
// i-navajo	                    IANA-registered language
// x-Newspeak	                private, unregistered language
// any-value-with-short-parts	although a schema processor will consider this value valid, it does not follow RFC 3066 guidelines
// Invalid values	    Comment
// longerThan8	        parts may not exceed 8 characters in length
//                      an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:language
pub struct Id<'a>(&'a str);