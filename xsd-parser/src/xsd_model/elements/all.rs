use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::complex_types::all_type;
use crate::xsd_model::elements::element::Element;

// xsd:all
// An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements. This declaration is for an "all" group that is a child of xsd:group; its type disallows minOccurs and maxOccurs
// See http://www.w3.org/TR/xmlschema-1/#element-all.
//
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Other elements with the same name: xsd:all
// Type: Anonymous
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      Choice [0..*]                   from group xsd:allModel
//          xsd:element
//
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
//
// Used in
// Type xsd:namedGroup (Element xsd:group)
pub struct All<'a> {
    annotation: Option<Annotation<'a>>,
    elements: Vec<Element<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
}


// xsd:all
// An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
// See http://www.w3.org/TR/xmlschema-1/#element-all.
//
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Other elements with the same name: xsd:all
// Type: xsd:allType
// Properties: Global, Qualified
//
// Used in
// Group xsd:typeDefParticle
// Group xsd:complexTypeModel via reference to xsd:typeDefParticle
// Type xsd:complexType via reference to xsd:complexTypeModel
// Type xsd:complexRestrictionType via reference to xsd:typeDefParticle (Element xsd:restriction)
// Type xsd:extensionType via reference to xsd:typeDefParticle (Element xsd:extension)
// Type xsd:localComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:topLevelComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
pub type AllType<'a> = all_type::AllType<'a>;

