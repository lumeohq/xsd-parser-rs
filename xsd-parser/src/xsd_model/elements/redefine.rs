use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::redefinable::Redefinable;
use crate::xsd_model::simple_types::any_uri::AnyUri;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:redefine
// See http://www.w3.org/TR/xmlschema-1/#element-redefine.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Choice [0..*]
//      xsd:annotation
//              from group xsd:redefinable
//      xsd:simpleType
//      xsd:complexType
//      xsd:group
//      xsd:attributeGroup
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// schemaLocation	[1..1]	xsd:anyURI
// id	            [0..1]	xsd:ID
//
// Used in
// Anonymous type of element xsd:schema
pub struct Redefine<'a> {
    annotations: Vec<Annotation<'a>>,
    content: Vec<Redefinable<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    schema_location: AnyUri<'a>,
    id: Id<'a>,
}
