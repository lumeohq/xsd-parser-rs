use crate::parser::constants::attribute;

#[derive(Debug, PartialEq)]
pub enum ElementType {
    All,
    Annotation,
    Any,
    AnyAttribute,
    AppInfo,
    Attribute,
    AttributeGroup,
    Choice,
    ComplexContent,
    ComplexType,
    Documentation,
    Element,
    Extension(ExtensionType),
    Field,
    Group,
    Import,
    Include,
    Key,
    KeyRef,
    List,
    Notation,
    Redefine,
    Restriction(RestrictionType),
    Schema,
    Selector,
    Sequence,
    SimpleContent,
    SimpleType,
    Union,
    Unique,
    Facet(FacetType),
    UnknownElement(String),
    XsdError(String),
}

#[derive(Debug, PartialEq)]
pub enum FacetType {
    Enumeration,
    FractionDigits,
    Length,
    MaxExclusive,
    MaxInclusive,
    MaxLength,
    MinExclusive,
    MinInclusive,
    MinLength,
    Pattern,
    TotalDigits,
    WhiteSpace,
}

#[derive(Debug, PartialEq)]
pub enum ExtensionType {
    ComplexContent,
    SimpleContent,
}

#[derive(Debug, PartialEq)]
pub enum RestrictionType {
    ComplexContent,
    SimpleContent,
    SimpleType,
}

pub trait XsdNode {
    fn xsd_type(&self) -> ElementType;
    fn attr_name(&self) -> Option<&str>;
    fn attr_type(&self) -> Option<&str>;
    fn attr_ref(&self) -> Option<&str>;
    fn attr_use(&self) -> UseType;
}

impl<'a> XsdNode for roxmltree::Node<'a, '_> {
    fn xsd_type(&self) -> ElementType {
        use ElementType::*;
        match self.tag_name().name() {
            "all" => All,
            "annotation" => Annotation,
            "any" => Any,
            "anyAttribute" => AnyAttribute,
            "appInfo" => AppInfo,
            "attribute" => Attribute,
            "attributeGroup" => AttributeGroup,
            "choice" => Choice,
            "complexContent" => ComplexContent,
            "complexType" => ComplexType,
            "documentation" => Documentation,
            "element" => Element,
            "extension" => match self.parent_element() {
                Some(parent) => match parent.tag_name().name() {
                    "complexContent" => Extension(ExtensionType::ComplexContent),
                    "simpleContent" => Extension(ExtensionType::SimpleContent),
                    _ => XsdError(format!(
                        "Unsupported parent type ({}) for 'extension' element",
                        parent.tag_name().name()
                    )),
                },
                None => XsdError("'extension' element must have a parent".to_string()),
            },
            "field" => Field,
            "group" => Group,
            "import" => Import,
            "include" => Include,
            "key" => Key,
            "keyRef" => KeyRef,
            "list" => List,
            "notation" => Notation,
            "redefine" => Redefine,
            "restriction" => match self.parent_element() {
                Some(parent) => match parent.tag_name().name() {
                    "complexContent" => Restriction(RestrictionType::ComplexContent),
                    "simpleContent" => Restriction(RestrictionType::SimpleContent),
                    "simpleType" => Restriction(RestrictionType::SimpleType),
                    _ => XsdError(format!(
                        "Unsupported parent type ({}) for 'restriction' element",
                        parent.tag_name().name()
                    )),
                },
                None => XsdError("'restriction' element must have a parent".to_string()),
            },
            "schema" => Schema,
            "selector" => Selector,
            "sequence" => Sequence,
            "simpleContent" => SimpleContent,
            "simpleType" => SimpleType,
            "union" => Union,
            "unique" => Unique,

            "enumeration" => Facet(FacetType::Enumeration),
            "fractionDigits" => Facet(FacetType::FractionDigits),
            "length" => Facet(FacetType::Length),
            "maxExclusive" => Facet(FacetType::MaxExclusive),
            "maxInclusive" => Facet(FacetType::MaxInclusive),
            "maxLength" => Facet(FacetType::MaxLength),
            "minExclusive" => Facet(FacetType::MinExclusive),
            "minInclusive" => Facet(FacetType::MinInclusive),
            "minLength" => Facet(FacetType::MinLength),
            "pattern" => Facet(FacetType::Pattern),
            "totalDigits" => Facet(FacetType::TotalDigits),
            "whiteSpace" => Facet(FacetType::WhiteSpace),

            _ => UnknownElement(self.tag_name().name().to_string()),
        }
    }

    fn attr_name(&self) -> Option<&str> {
        self.attribute(attribute::NAME)
    }

    fn attr_use(&self) -> UseType {
        match self.attribute(attribute::USE).unwrap_or("optional") {
            "optional" => UseType::Optional,
            "prohibited" => UseType::Prohibited,
            "required" => UseType::Required,
            _ => unreachable!(
                "If 'use' specified, this attribute must have one of the following values [optional, prohibited, required]"
            )
        }
    }

    fn attr_type(&self) -> Option<&str> {
        self.attribute(attribute::TYPE)
    }

    fn attr_ref(&self) -> Option<&str> {
        self.attribute(attribute::REF)
    }
}

pub enum UseType {
    Required,
    Prohibited,
    Optional,
}

pub type MinOccurs = usize;
pub enum MaxOccurs {
    Bounded(usize),
    Unbounded,
    None,
}

pub fn min_occurs(node: &roxmltree::Node) -> MinOccurs {
    node.attribute(attribute::MIN_OCCURS)
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(1)
}

pub fn max_occurs(node: &roxmltree::Node) -> MaxOccurs {
    match node.attribute(attribute::MAX_OCCURS) {
        Some(s) => match s {
            "unbounded" => MaxOccurs::Unbounded,
            s => s
                .parse::<usize>()
                .ok()
                .map(|val| MaxOccurs::Bounded(val))
                .unwrap_or(MaxOccurs::None),
        },
        None => MaxOccurs::None,
    }
}
