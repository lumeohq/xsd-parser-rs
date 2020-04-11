// xsd:blockSet
// #all or (possibly empty) subset of {substitution, extension, restriction}
// Simple type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Union of:
//      Type based on xsd:token
//          Valid value
//          #all
//      List of:
//          Type based on xsd:NMTOKEN
//              Valid value
//              extension
//              restriction
//              substitution
//
// Used by
// Attribute block
// Attribute blockDefault
pub enum BlockSet {
    All,
    List(BlockSetChoice)
}

pub enum BlockSetChoice {
    Extension,
    Restriction,
    Substitution,
}