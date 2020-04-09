use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{AnyUri, Id};
use crate::xsd_model::group::Group;
use crate::xsd_model::attribute_group::AttributeGroup;
use crate::xsd_model::simple_type::LocalSimpleType;


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

// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// schemaLocation	[1..1]	xsd:anyURI
// id	            [0..1]	xsd:ID
pub struct Redefine<'a> {
    annotations: Vec<Annotation<'a>>,
    content: Vec<RedefinableGroup<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    schema_location: AnyUri<'a>,
    id: Option<Id<'a>>
}

//This group is for the elements which can self-redefine.
pub enum RedefinableGroup<'a> {
    SimpleType(LocalSimpleType<'a>),
    //ComplexType(ComplexType), //TODO: add
    Group(Group<'a>),
    AttributeGroup(AttributeGroup<'a>),
}