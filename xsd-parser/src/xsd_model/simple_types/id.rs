// xsd:ID
// The type xsd:ID is used for an attribute that uniquely identifies an element in an XML document. An xsd:ID value must be an NCName. This means that it must start with a letter or underscore, and can only contain letters, digits, underscores, hyphens, and periods.
//
// xsd:ID carries several additional constraints:
//
// Their values must be unique within an XML instance, regardless of the attribute's name or its element name.
// A complex type cannot include more than one attribute of type xsd:ID, or any type derived from xsd:ID.
// xsd:ID attributes cannot have default or fixed values specified.
// Simple Type Information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema Document: datatypes.xsd
//
// Content
// Based on xsd:NCName
// Pattern: [\i-[:]][\c-[:]]* (Defined in type xsd:NCName)
// White Space: collapse (Defined in type xsd:token)
//
// Type Inheritance Chain
//  xsd:anySimpleType
//      restricted by xsd:string
//          restricted by xsd:normalizedString
//              restricted by xsd:token
//                  restricted by xsd:Name
//                      restricted by xsd:NCName
//                          restricted by xsd:ID
#[derive(Debug)]
pub struct Id<'a>(pub &'a str);
