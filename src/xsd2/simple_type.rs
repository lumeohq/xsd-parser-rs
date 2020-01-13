use std::fmt;
use crate::xsd2::utils::*;

pub struct SimpleType<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> SimpleType<'a, 'input> {

    pub fn name(&self) -> Option<&'a str> {
        self.node.attribute("name")
    }

    pub fn documentation(&self) -> Option<&'a str> {
        get_documentation(&self.node)
    }

    pub fn restriction(&self) -> Option<Restriction<'a, 'input>> {
        find_child(&self.node, "restriction").map(|node| Restriction{node})
    }

    pub fn list(&self) -> Option<List<'a, 'input>> {
        find_child(&self.node, "list").map(|node| List{node})
    }
}

impl<'a, 'input> fmt::Display for SimpleType<'a, 'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}  //{:?}", self.name(), self.documentation())
    }
}


pub struct Restriction<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input> Restriction<'a, 'input> {
    pub fn base(&self) -> &'a str {
        self.node.attribute("base").expect("Base required attribute")
    }

    pub fn facets(&self) -> Vec<Facet<'a, 'input>> {
        self.node.
            children().
            filter(|n| n.is_element()).
            map(|n| Facet::new(n)).
            collect()
    }
}

pub struct Facet<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
    pub facet_type: FacetType
}

impl<'a, 'input: 'a> Facet<'a, 'input> {
    pub fn value(&self) -> &'a str {
        self.node.attribute("value").expect("Facet value requiered")
    }

    pub fn documentation(&self) -> Option<&'a str> {
        get_documentation(&self.node)
    }

    pub fn new(node: roxmltree::Node<'a, 'input>) -> Self {
        let facet_type = match node.tag_name().name() {
            "enumeration"    => FacetType::Enumeration,
            "fractionDigits" => FacetType::FractionDigits,
            "length"         => FacetType::Length,
            "maxExclusive"   => FacetType::MaxExclusive,
            "maxInclusive"   => FacetType::MaxInclusive,
            "maxLength"      => FacetType::MaxLength,
            "minExclusive"   => FacetType::MinExclusive,
            "minInclusive"   => FacetType::MinInclusive,
            "minLength"      => FacetType::MinLength,
            "pattern"        => FacetType::Pattern,
            "totalDigits"    => FacetType::TotalDigits,
            "whiteSpace"     => FacetType::WhiteSpace,
            _ => FacetType::UNKNOWN
        };

        Facet{
            facet_type,
            node
        }
    }
}

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
    UNKNOWN
}

pub struct List<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> List<'a, 'input> {
    pub fn item_type(&self) -> Option<&'a str> {
        self.node.attribute("itemType")
    }
}
