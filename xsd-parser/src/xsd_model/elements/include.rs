use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::simple_types::any_uri::AnyUri;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:include
// See http://www.w3.org/TR/xmlschema-1/#element-include.
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]    from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// schemaLocation	[1..1]	xsd:anyURI
//
// Used in
// Anonymous type of element xsd:schema
#[derive(Debug, Default)]
pub struct Include<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id<'a>,
    pub schema_location: AnyUri<'a>,
}
