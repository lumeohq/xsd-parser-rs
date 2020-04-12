use crate::xsd_model::simple_types::any_uri::AnyUri;
use crate::xsd_model::{RawAttribute, RawElement};

// xsd:appinfo
// See http://www.w3.org/TR/xmlschema-1/#element-appinfo.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
// Any text (mixed) content, intermingled with:
// Any element [0..*] Namespace: ##any, Process Contents: lax
//
// Attributes
// source	        [0..1]	xsd:anyURI
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax
//
// Used in
// Anonymous type of element xsd:annotation
#[derive(Debug, Default)]
pub struct AppInfo<'a> {
    pub text: Option<&'a str>,
    pub elements: Vec<RawElement<'a>>,
    pub source: Option<AnyUri<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
}
