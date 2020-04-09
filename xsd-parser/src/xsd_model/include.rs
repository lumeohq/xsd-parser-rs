use crate::xsd_model::xsd::*;
use crate::xsd_model::AnyAttribute;
use crate::xsd_model::annotation::Annotation;

// See http://www.w3.org/TR/xmlschema-1/#element-include.
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]    from type xsd:annotated

// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// schemaLocation	[1..1]	xsd:anyURI
pub struct Include<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub attributes: Vec<AnyAttribute<'a>>,
    pub id: Option<Id<'a>>,
    pub schema_location: AnyUri<'a>,
}
