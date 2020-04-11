use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, NCName};

// See http://www.w3.org/TR/xmlschema-1/#element-attributeGroup.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:namedAttributeGroup
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//
//      Choice [0..*]    from group xsd:attrDecls
//          xsd:attribute
//          xsd:attributeGroup
//          xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
pub struct AttributeGroup<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub content: Vec<AttributeGroupChoice<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Option<Id<'a>>,
    pub name: NCName<'a>
}

pub enum AttributeGroupChoice<'a> {
    Attribute,  // TODO: Add attribute
    AttributeGroup(Box<AttributeGroup<'a>>),
    AnyAttribute  // TODO: add
}