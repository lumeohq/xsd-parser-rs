// xsd:fullDerivationSet
// #all or (possibly empty) subset of {extension, restriction, list, union}
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
//              Valid value	Description
//              extension	    Extension is disallowed
//              restriction	    Restriction is disallowed
//              list	        Derivation by list is disallowed
//              union	        Derivation by union is disallowed
//
// Used by
// Attribute finalDefault
pub enum FullDerivationSet {
    All,
    List(Vec<FullDerivationSubSet>)
}

pub enum FullDerivationSubSet {
    Extension,
    Restriction,
    List,
    Union,
}
