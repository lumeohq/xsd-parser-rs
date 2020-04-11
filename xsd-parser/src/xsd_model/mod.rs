pub mod common_groups;
pub mod element;
pub mod facets;
pub mod import;
pub mod include;
pub mod list;
pub mod notation;
pub mod redefine;
pub mod schema;
pub mod sequence;
pub mod simple_content;
pub mod simple_type;
pub mod union;
pub mod unique;
pub mod key;
pub mod key_ref;
pub mod selector;

pub mod complex_types;
pub mod elements;
pub mod groups;
pub mod simple_types;





pub type RawAttribute<'a> = roxmltree::Attribute<'a>;
pub type RawElement<'a> = roxmltree::Node<'a, 'a>;


pub enum MaxOccurs {
    Bounded(usize),
    Unbounded,
}

pub type XPath<'a> = &'a str;
