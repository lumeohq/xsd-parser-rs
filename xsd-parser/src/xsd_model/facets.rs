use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, AnySimpleType, PositiveInteger, NonNegativeInteger};

// We should use a substitution group for facets, but
// that's ruled out because it would allow users to
// add their own, which we're not ready for yet.
//
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
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
pub enum FacetGroup<'a> {
    MinExclusive(Facet<'a>),
    MinInclusive(Facet<'a>),
    MaxExclusive(Facet<'a>),
    MaxInclusive(Facet<'a>),
    TotalDigits(TotalDigits<'a>),
    FractionDigits(NumFacet<'a>),
    Length(NumFacet<'a>),
    MinLength(NumFacet<'a>),
    MaxLength(NumFacet<'a>),
    Enumeration(Enumeration<'a>),
    WhiteSpace(WhiteSpace<'a>),
    Pattern(Pattern<'a>)
}


// See http://www.w3.org/TR/xmlschema-2/#element-minExclusive.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:facet
// Properties: Global, Qualified
//
// Content
// from type xsd:annotated
// xsd:annotation [0..1]
//
// Attributes
// Any attribute	[0..*]		               Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                    from type xsd:annotated
// value	        [1..1]	xsd:anySimpleType
// fixed	        [0..1]	xsd:boolean		   Default value is "false".
//xsd:minExclusive, xsd:minInclusive, xsd:maxExclusive, xsd:maxInclusive
#[derive(Default)]
pub struct Facet<'a> {
    annotation: Option<Annotation<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    value: AnySimpleType<'a>,
    fixed: bool,
}


// See http://www.w3.org/TR/xmlschema-2/#element-totalDigits.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]
//
// Attributes
// id	            [0..1]	xsd:ID		                                                    from type xsd:annotated
// fixed	        [0..1]	xsd:boolean		    Default value is "false".                   from type xsd:facet
// value	        [1..1]	xsd:positiveInteger
// Any attribute	[0..*]		                Namespace: ##other, Process Contents: lax
pub struct TotalDigits<'a>{
    annotation: Option<Annotation<'a>>,
    id: Id<'a>,
    fixed: bool,
    value: PositiveInteger,
    attributes: Vec<RawAttribute<'a>>,
}


// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
//
// Content
// xsd:annotation [0..1]
//
// Attributes
// id	            [0..1]	xsd:ID		                                                        from type xsd:annotated
// fixed	        [0..1]	xsd:boolean		        Default value is "false".                   from type xsd:facet
// value	        [1..1]	xsd:nonNegativeInteger
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax
// xsd:fractionDigits xsd:length xsd:maxLength xsd:minLength
pub struct NumFacet<'a>{
    annotation: Option<Annotation<'a>>,
    id: Id<'a>,
    fixed: bool,
    value: NonNegativeInteger,
    attributes: Vec<RawAttribute<'a>>,
}


// See http://www.w3.org/TR/xmlschema-2/#element-enumeration.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:noFixedFacet
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]
//
// Attributes
// id	            [0..1]	xsd:ID		                                                    from type xsd:annotated
// value	        [1..1]	xsd:anySimpleType		                                        from type xsd:facet
// Any attribute	[0..*]		                Namespace: ##other, Process Contents: lax
pub struct Enumeration<'a>{
    annotation: Option<Annotation<'a>>,
    id: Id<'a>,
    value: AnySimpleType<'a>,
    attributes: Vec<RawAttribute<'a>>,
}


// See http://www.w3.org/TR/xmlschema-2/#element-whiteSpace.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]
//
// Attributes
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// fixed	        [0..1]	xsd:boolean	Default value is "false".                   from type xsd:facet
// value	        [1..1]	Anonymous (collapse | preserve | replace)
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax
pub struct WhiteSpace<'a>{
    annotation: Option<Annotation<'a>>,
    id: Id<'a>,
    fixed: bool,
    value: WhiteSpaceChoice,
    attributes: Vec<RawAttribute<'a>>,
}

pub enum WhiteSpaceChoice {
    Collapse,
    Preserve,
    Replace
}


// See http://www.w3.org/TR/xmlschema-2/#element-pattern.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]
//
// Attributes
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// value	        [1..1]	xsd:string
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax
pub struct Pattern<'a>{
    annotation: Option<Annotation<'a>>,
    id: Id<'a>,
    value: &'a str,
    attributes: Vec<RawAttribute<'a>>,
}