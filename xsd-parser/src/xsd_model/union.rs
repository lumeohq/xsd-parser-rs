use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::simple_type::LocalSimpleType;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, QName};

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      xsd:simpleType [0..*]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// memberTypes	    [0..1]	Anonymous
pub struct Union<'a> {
    annotation: Option<Annotation<'a>>,
    simple_type: Vec<Box<LocalSimpleType<'a>>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    memberTypes: Vec<QName<'a>>
}