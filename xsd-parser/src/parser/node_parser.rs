use roxmltree::Node;

use crate::parser::any::parse_any;
use crate::parser::any_attribute::parse_any_attribute;
use crate::parser::attribute::parse_attribute;
use crate::parser::attribute_group::parse_attribute_group;
use crate::parser::choice::parse_choice;
use crate::parser::complex_content::parse_complex_content;
use crate::parser::complex_type::parse_complex_type;
use crate::parser::element::parse_element;
use crate::parser::extension::parse_extension;
use crate::parser::import::parse_import;
use crate::parser::list::parse_list;
use crate::parser::restriction::parse_restriction;
use crate::parser::sequence::parse_sequence;
use crate::parser::simple_content::parse_simple_content;
use crate::parser::simple_type::parse_simple_type;
use crate::parser::types::RsEntity;
use crate::parser::union::parse_union;
use crate::parser::xsd_elements::{ElementType, XsdNode};

pub fn parse_node(node: &Node, parent: &Node) -> RsEntity {
    use ElementType::*;

    match node.xsd_type() {
        Any => parse_any(node),
        AnyAttribute => parse_any_attribute(node),
        Attribute => parse_attribute(node, parent),
        AttributeGroup => parse_attribute_group(node, parent),
        Choice => parse_choice(node),
        ComplexContent => parse_complex_content(node),
        ComplexType => parse_complex_type(node, parent),
        Element => parse_element(node, parent),
        Extension(_) => parse_extension(node, parent),
        Import | Include => parse_import(node),
        List => parse_list(node),
        Restriction(_) => parse_restriction(node, parent),
        Sequence => parse_sequence(node, parent),
        SimpleContent => parse_simple_content(node),
        SimpleType => parse_simple_type(node, parent),
        Union => parse_union(node),

        _ => unreachable!(
            "Unsupported node:\n {:?}\nparent = {:?}\n",
            node,
            node.parent()
        ),
    }
}
