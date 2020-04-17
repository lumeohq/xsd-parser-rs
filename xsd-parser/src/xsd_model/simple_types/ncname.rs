// xsd:NCName
// The type xsd:NCName represents an XML non-colonized name, which is simply a name that does not contain colons. An xsd:NCName value must start with either a letter or underscore (_) and may contain only letters, digits, underscores (_), hyphens (-), and periods (.). This is equivalent to the Name type, except that colons are not permitted.
//
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:Name
// Pattern: [\i-[:]][\c-[:]]*
// White Space: collapse (Defined in type xsd:token)
//
// Examples
// Valid values	        Comment
// myElement
// _my.Element
// my-element
// Invalid values	    Comment
// pre:myElement	    an NCName must not contain a colon
// -myelement	        an NCName must not start with a hyphen
//                      an empty value is not valid, unless xsi:nil is used
//
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:Name
//                      restricted by xsd:NCName
//                          restricted by xsd:ID
//                              restricted by xsd:IDREF
//                                  used in list xsd:IDREFS
//                              restricted by xsd:ENTITY
//                                  used in list xsd:ENTITIES
#[derive(Debug)]
pub struct NCName<'a>(&'a str);
