use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::elements::simple_type::LocalSimpleType;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:notation
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
//
// Used in
// Group xsd:schemaTop
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
#[derive(Default, Debug)]
pub struct Union<'a> {
    annotation: Option<Annotation<'a>>,
    simple_type: Vec<LocalSimpleType<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    member_types: Vec<QName<'a>>,
}
