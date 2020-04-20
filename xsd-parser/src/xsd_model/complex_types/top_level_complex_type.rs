use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::groups::complex_type_model::ComplexTypeModel;
use crate::xsd_model::groups::type_def_particle::TypeDefParticle;
use crate::xsd_model::simple_types::derivation_set::DerivationSet;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::{ComplexContent, RawAttribute, SimpleContent};

// xsd:topLevelComplexType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      Choice [1..1]    from group xsd:complexTypeModel
//          xsd:simpleContent
//          xsd:complexContent
//          Sequence [1..1]
//              Choice [0..1]    from group xsd:typeDefParticle
//                  xsd:group
//                  xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//                  xsd:choice
//                  xsd:sequence
//              Choice [0..*]    from group xsd:attrDecls
//                  xsd:attribute
//                  xsd:attributeGroup
//              xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		                Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                    from type xsd:annotated
// name	            [1..1]	xsd:NCName
// abstract	        [0..1]	xsd:boolean		                                                Default value is "false".
// final	        [0..1]	xsd:derivationSet
// block	        [0..1]	xsd:derivationSet
// mixed	        [0..1]	xsd:boolean	        Indicates that mixed content is allowed.	Default value is "false".
//
// Used by
// Element xsd:complexType
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:topLevelComplexType
#[derive(Debug)]
pub struct TopLevelComplexType<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub model: ComplexTypeModel<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id<'a>,
    pub name: QName<'a>,
    pub abstract_: bool,
    pub final_: Option<DerivationSet>,
    pub block: Option<DerivationSet>,
    pub mixed: bool,
}

impl<'a> TopLevelComplexType<'a> {
    pub fn simple_content(&self) -> Option<&SimpleContent<'a>> {
        match &self.model {
            ComplexTypeModel::SimpleContent(sc) => Some(sc),
            _ => None,
        }
    }

    pub fn complex_content(&self) -> Option<&ComplexContent<'a>> {
        match &self.model {
            ComplexTypeModel::ComplexContent(sc) => Some(sc),
            _ => None,
        }
    }

    pub fn type_def_particle(&self) -> Option<&TypeDefParticle<'a>> {
        match &self.model {
            ComplexTypeModel::Content(tdp, _) => tdp.as_ref(),
            _ => None,
        }
    }

    pub fn attr_decls(&self) -> Option<&AttrDecls<'a>> {
        match &self.model {
            ComplexTypeModel::Content(_, attr_decls) => Some(attr_decls),
            _ => None,
        }
    }
}
