use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::AnyAttribute;
use crate::xsd_model::xsd::{Id, NCName, Public, AnyUri};

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
pub struct Notation<'a> {
    annotation: Option<Annotation<'a>>,
    attributes: Vec<AnyAttribute<'a>>,
    id: Option<Id<'a>>,
    name: NCName<'a>,
    public: Option<Public<'a>>,
    system: Option<AnyUri<'a>>,
}

