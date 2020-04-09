use crate::xsd_model::xsd::{Id, AnyUri};
use crate::xsd_model::RawAttribute;

// See http://www.w3.org/TR/xmlschema-1/#element-import.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
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
pub struct Import<'a> {
    annotation: Option<Id<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Option<Id<'a>>,
    namespace: Option<AnyUri<'a>>,
    schema_location: Option<AnyUri<'a>>
}