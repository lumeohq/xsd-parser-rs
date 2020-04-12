use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::simple_types::any_uri::AnyUri;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::simple_types::public::Public;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:notation
// See http://www.w3.org/TR/xmlschema-1/#element-notation.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
// xsd:annotation [0..1]   from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
// public	        [0..1]	xsd:public
// system	        [0..1]	xsd:anyURI
//
// Used in
// Group xsd:schemaTop
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
pub struct Notation<'a> {
    annotation: Option<Annotation<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    name: NCName<'a>,
    public: Option<Public<'a>>,
    system: Option<AnyUri<'a>>,
}
