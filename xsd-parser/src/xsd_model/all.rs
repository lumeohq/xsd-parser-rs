use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::{RawAttribute, MaxOccurs};
use crate::xsd_model::xsd::{NonNegativeInteger, Id};

// An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
// See http://www.w3.org/TR/xmlschema-1/#element-all.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:allType
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      Choice [0..*]                   from group xsd:allModel
//          xsd:element
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// minOccurs	    [0..1]	Anonymous		Default value is "1".
// maxOccurs	    [0..1]	Anonymous		Default value is "1".
pub struct All<'a> {
    annotation: Option<Annotation<'a>>,
    elements: Vec<Element<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    min_occurs: NonNegativeInteger,
    max_occurs: MaxOccurs,
}