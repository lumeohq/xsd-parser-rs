use crate::xsd_model::simple_types::token::Token;

// xsd:public
// A public identifier, per ISO 8879
// Simple type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  xsd:token
//
// Used by
// Attribute public
//
// Type inheritance chain
//  xsd:anySimpleType
//      xsd:string
//          xsd:normalizedString
//              xsd:token
//                  xsd:public
pub type Public<'a> = Token<'a>;
