use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, NCName};

// See http://www.w3.org/TR/xmlschema-1/#element-group.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Other elements with the same name: xsd:group
// Type: xsd:namedGroup
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]     from type xsd:annotated
//      Choice [1..1]
//          xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements. This declaration is for an "all" group that is a child of xsd:group; its type disallows minOccurs and maxOccurs
//          xsd:choice
//          xsd:sequence
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
pub struct Group<'a> {
    annotation: Option<Annotation<'a>>,
    content_choice: ContentChoice,
    attributes: Vec<RawAttribute<'a>>,
    id: Option<Id<'a>>,
    name: NCName<'a>
}

//TODO: add
pub enum ContentChoice {
    All,
    Choice,
    Sequence,
}