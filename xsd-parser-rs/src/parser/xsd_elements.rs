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

#[derive(Debug, Clone, PartialEq)]
pub enum FacetType {
    Enumeration(String),
    FractionDigits(String),
    Length(String),
    MaxExclusive(String),
    MaxInclusive(String),
    MaxLength(String),
    MinExclusive(String),
    MinInclusive(String),
    MinLength(String),
    Pattern(String),
    TotalDigits(String),
    WhiteSpace(WhiteSpace),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhiteSpace {
    Preserve,
    Replace,
    Collapse,
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
    fn attr_value(&self) -> Option<&str>;
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

            "enumeration" => Facet(FacetType::Enumeration(get_string_value(self))),
            "fractionDigits" => Facet(FacetType::FractionDigits(get_string_value(self))),
            "length" => Facet(FacetType::Length(get_string_value(self))),
            "maxExclusive" => Facet(FacetType::MaxExclusive(get_string_value(self))),
            "maxInclusive" => Facet(FacetType::MaxInclusive(get_string_value(self))),
            "maxLength" => Facet(FacetType::MaxLength(get_string_value(self))),
            "minExclusive" => Facet(FacetType::MinExclusive(get_string_value(self))),
            "minInclusive" => Facet(FacetType::MinInclusive(get_string_value(self))),
            "minLength" => Facet(FacetType::MinLength(get_string_value(self))),
            "pattern" => Facet(FacetType::Pattern(get_string_value(self))),
            "totalDigits" => Facet(FacetType::TotalDigits(get_string_value(self))),
            "whiteSpace" => match self.attr_value() {
                Some(val) => match val {
                    "preserve" => Facet(FacetType::WhiteSpace(WhiteSpace::Preserve)),
                    "replace" => Facet(FacetType::WhiteSpace(WhiteSpace::Replace)),
                    "collapse" => Facet(FacetType::WhiteSpace(WhiteSpace::Collapse)),
                    x => unreachable!("Invalid WhiteSpace value: {}.\n {:?}", x, self),
                },
                None => unreachable!("Value is required for facets"),
            },

            _ => UnknownElement(self.tag_name().name().to_string()),
        }
    }

    fn attr_name(&self) -> Option<&str> {
        self.attribute(attribute::NAME)
    }

    fn attr_type(&self) -> Option<&str> {
        self.attribute(attribute::TYPE)
    }

    fn attr_ref(&self) -> Option<&str> {
        self.attribute(attribute::REF)
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

    fn attr_value(&self) -> Option<&str> {
        self.attribute(attribute::VALUE)
    }
}

fn get_string_value(node: &roxmltree::Node) -> String {
    node.attr_value()
        .map(|s| s.to_string())
        .unwrap_or_else(|| panic!("Value is required. {:?}", node))
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
                .map(MaxOccurs::Bounded)
                .unwrap_or(MaxOccurs::None),
        },
        None => MaxOccurs::None,
    }
}
