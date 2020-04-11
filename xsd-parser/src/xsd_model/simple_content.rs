use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::Id;

// See http://www.w3.org/TR/xmlschema-1/#element-simpleContent.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      Choice [1..1]
//          xsd:restriction
//          xsd:extension
//
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
pub struct SimpleContent<'a> {
    annotation: Option<Annotation<'a>>,
    content: SimpleContentChoice<'a>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>
}


pub enum SimpleContentChoice<'a> {
    Restriction(SimpleRestrictionType<'a>),
    Extension(SimpleExtensionType<'a>)
}