pub mod annotation;
pub mod any;
pub mod any_attribute;
pub mod app_info;
pub mod attr_decls;
pub mod attribute;
pub mod complex_type;
pub mod complex_type_model;
pub mod documentation;
pub mod explicit_group;
pub mod facets;
pub mod group;
pub mod import;
pub mod include;
pub mod list;
pub mod nested_particle;
pub mod restriction;
pub mod schema;
pub mod schema_top;
pub mod simple_content;
pub mod simple_extension;
pub mod simple_restriction;
pub mod simple_restriction_model;
pub mod simple_type;
pub mod type_def_particle;
pub mod union;
pub mod utils;

use crate::xsd_model::elements::{xsd_element_type, ElementType};
use crate::xsd_model::simple_types::any_uri::AnyUri;
use crate::xsd_model::simple_types::id::Id;
use crate::xsd_model::simple_types::language::Language;
use crate::xsd_model::simple_types::ncname::NCName;
use roxmltree::{Attribute, Children, Node};
use std::str::FromStr;

pub const XSD_NS_URI: &str = "http://www.w3.org/2001/XMLSchema";

pub trait XsdNode {
    fn xsd_type(&self) -> Result<ElementType, String>;
}

impl XsdNode for roxmltree::Node<'_, '_> {
    fn xsd_type(&self) -> Result<ElementType, String> {
        if let Some(uri) = self.tag_name().namespace() {
            if uri != XSD_NS_URI {
                return Err(format!(
                    "Invalid prefix for xsd element: {:?}",
                    self.tag_name()
                ));
            }
        }

        xsd_element_type(self.tag_name().name())
    }
}

macro_rules! impl_from_attr {
    ($type_name:ident) => {
        impl<'a> From<&'a Attribute<'a>> for $type_name<'a> {
            fn from(a: &'a Attribute<'a>) -> Self {
                $type_name(a.value())
            }
        }
    };
}

impl_from_attr!(AnyUri);
impl_from_attr!(Language);
impl_from_attr!(Id);
impl_from_attr!(NCName);

pub enum GroupErr<'a> {
    ElementParsing(String),
    InvalidNode(Node<'a, 'a>),
}

impl From<String> for GroupErr<'_> {
    fn from(s: String) -> Self {
        GroupErr::ElementParsing(s)
    }
}

pub trait GroupResultConvert<T> {
    fn into(&self) -> Result<T, String>;
}

#[derive(Clone)]
pub struct ElementChildren<'a, 'input: 'a> {
    front: Option<Node<'a, 'input>>,
    back: Option<Node<'a, 'input>>,
}

impl<'a, 'input: 'a> Iterator for ElementChildren<'a, 'input> {
    type Item = Node<'a, 'input>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            let node = self.front.take();
            self.back = None;
            node
        } else {
            let node = self.front.take();
            self.front = node.as_ref().and_then(Node::next_sibling_element);
            node
        }
    }
}

impl<'a, 'input: 'a> ElementChildren<'a, 'input> {
    #[inline]
    fn prev(&mut self) -> Option<Node<'a, 'input>> {
        let node = self.front.take();
        self.front = node.as_ref().and_then(Node::prev_sibling_element);
        node
    }
}

pub trait ElementChildren_<'a, 'input: 'a> {
    fn element_children(&self) -> ElementChildren<'a, 'input>;
}

impl<'a, 'input: 'a> ElementChildren_<'a, 'input> for Node<'a, 'input> {
    fn element_children(&self) -> ElementChildren<'a, 'input> {
        ElementChildren {
            front: self.first_element_child(),
            back: self.last_element_child(),
        }
    }
}
