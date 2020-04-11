use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::common_groups::ComplexTypeModel;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, QName};
use crate::xsd_model::simple_type::SimpleDerivationSet;
use crate::xsd_model::complex_types::top_level_complex_type;
use crate::xsd_model::complex_types::local_complex_type;

// See http://www.w3.org/TR/xmlschema-1/#element-complexType.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:topLevelComplexType
// Properties: Global, Qualified
//
// Used in
// Group xsd:redefinable
// Anonymous type of element xsd:redefine via reference to xsd:redefinable
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
// Group xsd:schemaTop via reference to xsd:redefinable
pub type TopLevelComplexType<'a> = top_level_complex_type::TopLevelComplexType<'a>;


// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:localComplexType
// Properties: Local, Qualified
//
// Used in
// Group xsd:elementModel
// Type xsd:localElement via reference to xsd:elementModel (Element xsd:element)
// Type xsd:narrowMaxMin via reference to xsd:elementModel (Element xsd:element)
// Type xsd:topLevelElement via reference to xsd:elementModel (Element xsd:element)
pub type LocalComplexType_<'a> = local_complex_type::LocalComplexType<'a>;