// xsd:token
// The type xsd:token represents a character string that may contain any Unicode character allowed by XML.
// Certain characters, namely the "less than" symbol (<) and the ampersand (&), must be escaped
// (using the entities &lt; and &amp;, respectively) when used in strings in XML instances.
// The name xsd:token may be slightly confusing because it implies that there may be only one token
// with no whitespace. In fact, there can be whitespace in a token value. The xsd:token type has a
// whiteSpace facet of collapse, which means that the processor replaces each carriage return,
// line feed, and tab by a single space. After this replacement, each group of consecutive spaces
// is collapsed into one space character, and all leading and trailing spaces are removed.
// This processing is equivalent to the processing of non-CDATA attribute values in XML 1.0.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
//  Based on xsd:normalizedString
//  White Space: collapse
// Examples
//
// Valid values	                Comment
// This is a string!
// Édition française.
// 12.5
//                              an empty string is valid
// PB&amp;J	                    when parsed, it will become "PB&J"
//    Separated by 3 spaces.	when parsed, it will become "Separated by 3 spaces."
// This
// is on two lines.	            when parsed, the line break will be replaced with a space
// Invalid values	            Comment
// AT&T	                        ampersand must be escaped
// 3 < 4	                    the "less than" symbol must be escaped
//
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:language
//                  restricted by xsd:NMTOKEN
//                      used in list xsd:NMTOKENS
//                  restricted by xsd:Name
//                      restricted by xsd:NCName
//                          restricted by xsd:ID
//                          restricted by xsd:IDREF
//                              used in list xsd:IDREFS
//                          restricted by xsd:ENTITY
//                              used in list xsd:ENTITIES
pub struct Token<'a>(pub &'a str);
