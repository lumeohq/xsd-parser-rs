use crate::xsd_model::xsd::{AnyUri, Language, Id};
use crate::xsd_model::{RawAttribute, RawElement};


// See http://www.w3.org/TR/xmlschema-1/#element-annotation.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Choice [0..*]
//      xsd:appinfo
//      xsd:documentation
//  Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID
pub struct Annotation<'a> {
    pub app_infos: Vec<AppInfo<'a>>,
    pub documentations: Vec<Documentation<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Option<Id<'a>>
}


// See http://www.w3.org/TR/xmlschema-1/#element-appinfo.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
// Any text (mixed) content, intermingled with:
// Any element      [0..*]   Namespace: ##any, Process Contents: lax

// Attributes
// source	        [0..1]	xsd:anyURI
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax
pub struct AppInfo<'a> {
    pub text: &'a str,
    pub elements: Vec<RawElement<'a>>,
    pub source: Option<AnyUri<'a>>,
    pub attributes: Vec<RawAttribute<'a>>
}


// See http://www.w3.org/TR/xmlschema-1/#element-documentation.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Any text (mixed) content, intermingled with:
//  Any element     [0..*]  Namespace: ##any, Process Contents: lax
//
// Attributes
// source	        [0..1]	    xsd:anyURI
// xml:lang	        [0..1]	    Anonymous
// Any attribute	[0..*]		            Namespace: ##other, Process Contents: lax
pub struct Documentation<'a>{
    pub text: &'a str,
    pub elements: Vec<RawElement<'a>>,
    pub source: Option<AnyUri<'a>>,
    pub lang: Option<Language<'a>>,
    pub attributes: Vec<RawAttribute<'a>>
}