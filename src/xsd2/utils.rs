use roxmltree::*;
use crate::xsd2::sequence::Element;

pub fn find_child<'a, 'input>(node: &roxmltree::Node<'a, 'input>, tag_name: &str) -> Option<roxmltree::Node<'a, 'input>> {
    node.children().find(|e| e.is_element() && e.tag_name().name() == tag_name)
}

pub fn find_element<'a, 'input>(node: &roxmltree::Node<'a, 'input>, tag_name: &str) -> Option<roxmltree::Node<'a, 'input>> {
    match node.
        traverse().
        find(|e| match e {
            Edge::Open(x) => x.is_element() && x.tag_name().name() == tag_name,
            _ => false
        })
    {
        Some(Edge::Open(node)) => Some(node.clone()),
        _ => None
    }
}

pub fn get_documentation<'a>(node: &roxmltree::Node<'a, '_>) -> Option<&'a str> {
    find_child(node, "annotation").
        and_then(|node| find_child(&node, "documentation")).
        and_then(|node| node.text())
}

pub fn get_node_type<'a>(node: &roxmltree::Node<'a, '_>) -> &'a str {
    match node.attribute("type") {
        Some(name) => name,
        None => match node.attribute("ref") {
            Some(s) => s,
            None => "_UNSUPPORTED_TYPE"
        }
    }
}

pub fn get_node_name<'a>(node: &roxmltree::Node<'a, '_>) -> &'a str {
    match node.attribute("name") {
        Some(name) => name,
        None => match node.attribute("ref") {
            Some(s) => s,
            None => "_UNSUPPORTED_NAME"
        }
    }
}

pub type MinOccurs = usize;
pub enum MaxOccurs {
    Bounded(usize),
    Unbounded,
    None
}

pub trait Node {
    fn node(&self) -> & roxmltree::Node<'_, '_>;
}

pub trait MinMaxOccurs: Node {
    fn min_occurs(&self) -> MinOccurs {
        self.node()
            .attribute("minOccurs")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(1)
    }

    fn max_occurs(&self) -> MaxOccurs {
        match self.node().attribute("maxOccurs") {
            Some(s) => match s {
                "unbounded" => MaxOccurs::Unbounded,
                s => s.
                    parse::<usize>().
                    ok().
                    map(|val| MaxOccurs::Bounded(val)).
                    unwrap_or(MaxOccurs::None)
            },
            None => MaxOccurs::None
        }
    }
}

pub trait Elements: Node {
    fn elements(&self) -> Vec<Element> {
        self.node().
            children().
            filter(|node| node.is_element() && node.tag_name().name() == "element").
            map(|node| Element{node}).
            collect::<Vec<Element>>()
    }
}

pub type AnyAttribute<'a, 'input> = roxmltree::Node<'a, 'input>;

#[macro_export] macro_rules! create_node_type{
    ($name:ident) => {
        pub struct $name<'a, 'input> {
            pub node: roxmltree::Node<'a, 'input>,
        }

        impl Node for $name<'_, '_>{
           fn node(&self) -> &roxmltree::Node {
                &self.node
           }
        }
    };
    ($name:ident, $($ty:ty), *) => {
        create_node_type!{$name}
        $(impl $ty for $name<'_, '_>{})*
    };
}

