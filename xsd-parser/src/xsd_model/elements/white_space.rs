use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:whiteSpace
// See http://www.w3.org/TR/xmlschema-2/#element-whiteSpace.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  xsd:annotation [0..1]
//
// Attributes
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// fixed	        [0..1]	xsd:boolean	Default value is "false".                   from type xsd:facet
// value	        [1..1]	Anonymous (collapse | preserve | replace)
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax
//
// Used in
// Group xsd:facets
// Anonymous type of element xsd:restriction via reference to xsd:simpleRestrictionModel
// Group xsd:simpleRestrictionModel via reference to xsd:facets
// Type xsd:simpleRestrictionType via reference to xsd:simpleRestrictionModel (Element xsd:restriction)
pub struct WhiteSpace<'a>{
    annotation: Option<Annotation<'a>>,
    id: Id<'a>,
    fixed: bool,
    value: WhiteSpaceChoice,
    attributes: Vec<RawAttribute<'a>>,
}

pub enum WhiteSpaceChoice {
    Collapse,
    Preserve,
    Replace
}