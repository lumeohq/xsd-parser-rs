use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::elements::extension::Extension;
use crate::xsd_model::elements::restriction::ComplexRestriction;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:complexContent
// See http://www.w3.org/TR/xmlschema-1/#element-complexContent.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      Choice [1..1]
//          xsd:restriction
//          xsd:extension
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// mixed	        [0..1]	xsd:boolean	Indicates that mixed content is allowed.
//
// Used in
// Group xsd:complexTypeModel
// Type xsd:complexType via reference to xsd:complexTypeModel
// Type xsd:localComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:topLevelComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
#[derive(Debug)]
pub struct ComplexContent<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub content: ComplexContentChoice<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id<'a>,
    pub mixed: bool,
}

#[derive(Debug)]
pub enum ComplexContentChoice<'a> {
    Restriction(Box<ComplexRestriction<'a>>),
    Extension(Box<Extension<'a>>),
}
