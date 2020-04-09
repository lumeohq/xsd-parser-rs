use crate::xsd_model::include::Include;
use crate::xsd_model::import::Import;
use crate::xsd_model::redefine::Redefine;
use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{AnyUri, Token, Id, Language};
use crate::xsd_model::notation::Notation;
use crate::xsd_model::attribute_group::AttributeGroup;


// See http://www.w3.org/TR/xmlschema-1/#element-schema.
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      Choice [0..*]
//          xsd:include
//          xsd:import
//          xsd:redefine
//          xsd:annotation
//      Sequence [0..*]
//          Choice [1..1]   from group xsd:schemaTop
//                          from group xsd:redefinable
//              xsd:simpleType
//              xsd:complexType
//              xsd:group
//              xsd:attributeGroup

//              xsd:element
//              xsd:attribute
//              xsd:notation
//          xsd:annotation [0..*]

// Attributes
// Any attribute	    [0..*]		                    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// targetNamespace	    [0..1]	xsd:anyURI
// version	            [0..1]	xsd:token
// finalDefault	        [0..1]	xsd:fullDerivationSet	Default value is "".
// blockDefault	        [0..1]	xsd:blockSet		    Default value is "".
// attributeFormDefault	[0..1]	xsd:formChoice		    Default value is "unqualified".
// elementFormDefault	[0..1]	xsd:formChoice		    Default value is "unqualified".
// id	                [0..1]	xsd:ID
// xml:lang	            [0..1]	Anonymous

// Identity constraints
// Type	Name	            Selector	                        Field(s)
// key	element	            xs:element	                        @name
// key	attribute	        xs:attribute	                    @name
// key	type	            xs:complexType|xs:simpleType        @name
// key	group	            xs:group	                        @name
// key	attributeGroup 	    xs:attributeGroup	                @name
// key	notation	        xs:notation	                        @name
// key	identityConstraint	.//xs:key|.//xs:unique|.//xs:keyref	@name
pub struct Schema<'a> {
    pub includes: Vec<Include<'a>>,
    pub imports: Vec<Import<'a>>,
    pub redefines: Vec<Redefine<'a>>,
    pub annotations: Vec<Annotation<'a>>,
    pub content: Vec<(SchemaTopGroup<'a>, Option<Annotation<'a>>)>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub target_namespace: Option<AnyUri<'a>>,
    pub version: Option<Token<'a>>,
    pub final_default: Option<FullDerivationSet>,
    pub block_default: Option<BlockSet>,
    pub attribute_form_default: FormChoice,
    pub element_form_default: FormChoice,
    pub id: Option<Id<'a>>,
    pub lang: Option<Language<'a>>
}


pub enum SchemaTopGroup<'a> {
    //SimpleType(SimpleType),
    //ComplexType(ComplexType),
    //Group(Group),
    AttributeGroup(AttributeGroup<'a>),
    //Element(TopLevelElement),
    //Attribute(Attribute),
    Notation(Notation<'a>),
}

pub enum FullDerivationSet {
    All,
    Subset(Vec<DerivationSubset>)
}

pub enum DerivationSubset {
    Extension,
    Restriction,
    List,
    Union
}

pub enum BlockSet {
    All,
    Subset(Vec<BlockSubset>)
}

pub enum BlockSubset {
    Extension,
    Restriction,
    Substitution,
}

pub enum FormChoice{
    Qualified,
    Unqualified
}

impl Default for FormChoice {
    fn default() -> Self {
        FormChoice::Unqualified
    }
}