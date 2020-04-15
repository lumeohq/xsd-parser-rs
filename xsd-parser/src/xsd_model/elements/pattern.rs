use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:pattern
// See http://www.w3.org/TR/xmlschema-2/#element-pattern.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]
// Attributes
// id	            [0..1]	xsd:ID		from type xsd:annotated
// value	        [1..1]	xsd:string
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax
//
// Used in
// Group xsd:facets
// Anonymous type of element xsd:restriction via reference to xsd:simpleRestrictionModel
// Group xsd:simpleRestrictionModel via reference to xsd:facets
// Type xsd:simpleRestrictionType via reference to xsd:simpleRestrictionModel (Element xsd:restriction)
#[derive(Default, Debug)]
pub struct Pattern<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub id: Id<'a>,
    pub value: &'a str,
    pub attributes: Vec<RawAttribute<'a>>,
}
