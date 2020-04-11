// xsd:facets
//        We should use a substitution group for facets, but
//        that's ruled out because it would allow users to
//        add their own, which we're not ready for yet.
//
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: datatypes.xsd
//
// Content
//  Choice [1..1]
//      xsd:minExclusive
//      xsd:minInclusive
//      xsd:maxExclusive
//      xsd:maxInclusive
//      xsd:totalDigits
//      xsd:fractionDigits
//      xsd:length
//      xsd:minLength
//      xsd:maxLength
//      xsd:enumeration
//      xsd:whiteSpace
//      xsd:pattern
//
// Used in
// Group xsd:simpleRestrictionModel
pub enum Facets<'a> {
    // MinExclusive(Facet<'a>),
    // MinInclusive(Facet<'a>),
    // MaxExclusive(Facet<'a>),
    // MaxInclusive(Facet<'a>),
    // TotalDigits(TotalDigits<'a>),
    // FractionDigits(NumFacet<'a>),
    // Length(NumFacet<'a>),
    // MinLength(NumFacet<'a>),
    // MaxLength(NumFacet<'a>),
    // Enumeration(Enumeration<'a>),
    // WhiteSpace(WhiteSpace<'a>),
    // Pattern(Pattern<'a>)
}