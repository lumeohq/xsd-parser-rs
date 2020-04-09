use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::simple_type::LocalSimpleType;
use crate::xsd_model::facets::FacetGroup;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, QName};

// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]        from type xsd:annotated
//      xsd:simpleType [0..1]        from group xsd:simpleRestrictionModel
//      Choice [0..*]        from group xsd:facets
//          xsd:minExclusive
//          xsd:minInclusive
//          xsd:maxExclusive
//          xsd:maxInclusive
//          xsd:totalDigits
//          xsd:fractionDigits
//          xsd:length
//          xsd:minLength
//          xsd:maxLength
//          xsd:enumeration
//          xsd:whiteSpace
//          xsd:pattern
//
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
// base	            [0..1]	xsd:QName
pub struct SimpleTypeRestriction<'a>{
    annotation: Option<Annotation<'a>>,
    simple_type: Option<Box<LocalSimpleType<'a>>>,
    facets: Vec<FacetGroup<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    base: Option<QName<'a>>
}

