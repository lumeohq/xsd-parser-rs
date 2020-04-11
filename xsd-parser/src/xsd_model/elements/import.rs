use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::simple_types::any_uri::AnyUri;

// xsd:import
// See http://www.w3.org/TR/xmlschema-1/#element-import.
//
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Type: Anonymous
//
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]      from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// namespace	    [0..1]	xsd:anyURI
// schemaLocation	[0..1]	xsd:anyURI
//
// Used in
// Anonymous type of element xsd:schema
pub struct Import<'a> {
    annotation: Option<Annotation<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    namespace: Option<AnyUri<'a>>,
    schema_location: Option<AnyUri<'a>>
}