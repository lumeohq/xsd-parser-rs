// xsd:derivationSet
// #all or (possibly empty) subset of {extension, restriction}
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
//              Valid value	    Description
//              extension	    Extension is disallowed
//              restriction	    Restriction is disallowed
//
// Used by
// Attribute block
// Attribute final
pub enum DerivationSet {
    All,
    List(Vec<DerivationSubset>)
}

pub enum DerivationSubset {
    Extension,
    Restriction,
}