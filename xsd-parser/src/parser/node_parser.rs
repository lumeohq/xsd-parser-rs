use roxmltree::Node;

use crate::parser::{
    all::parse_all,
    any::parse_any,
    any_attribute::parse_any_attribute,
    attribute::parse_attribute,
    attribute_group::parse_attribute_group,
    choice::parse_choice,
    complex_content::parse_complex_content,
    complex_type::parse_complex_type,
    element::parse_element,
    extension::parse_extension,
    import::parse_import,
    list::parse_list,
    restriction::parse_restriction,
    sequence::parse_sequence,
    simple_content::parse_simple_content,
    simple_type::parse_simple_type,
    types::RsEntity,
    union::parse_union,
    xsd_elements::{ElementType, XsdNode},
};

pub fn parse_node(node: &Node, parent: &Node) -> RsEntity {
    use ElementType::*;

    match node.xsd_type() {
        All => parse_all(node, parent),
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

        _ => unreachable!("Unsupported node:\n {:?}\nparent = {:?}\n", node, node.parent()),
    }
}
