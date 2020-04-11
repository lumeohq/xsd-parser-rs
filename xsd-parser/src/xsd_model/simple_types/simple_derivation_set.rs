// xsd:simpleDerivationSet
//    #all or (possibly empty) subset of {restriction, union, list}
//
//    A utility type, not for public use
// Simple type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: datatypes.xsd
//
// Content
//  Union of:
//      Type based on xsd:token
//          Valid value
//          #all
//      List of:
//          Type based on xsd:NMTOKEN
//              Valid value
//                  list
//                  union
//                  restriction
//
// Used by
// Attribute final
pub enum SimpleDerivationSet {
    All,
    List(Vec<SimpleDerivationSubset>)
}

pub enum SimpleDerivationSubset {
    List,
    Union,
    Restriction,
}
