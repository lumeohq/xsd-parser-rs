pub mod all;
pub mod annotation;
pub mod any;
pub mod any_attribute;
pub mod attribute;
pub mod attribute_group;
pub mod choice;
pub mod common_groups;
pub mod complex_content;
pub mod complex_type;
pub mod element;
pub mod extension;
pub mod facets;
pub mod group;
pub mod import;
pub mod include;
pub mod list;
pub mod notation;
pub mod redefine;
pub mod restriction;
pub mod schema;
pub mod sequence;
pub mod simple_content;
pub mod simple_type;
pub mod union;
pub mod unique;
pub mod key;
pub mod key_ref;
pub mod selector;
pub mod field;

pub mod complex_types;
pub mod elements;
pub mod groups;
pub mod simple_types;





pub type RawAttribute<'a> = roxmltree::Attribute<'a>;
pub type RawElement<'a> = roxmltree::Node<'a, 'a>;

pub enum FormChoice{
    Qualified,
    Unqualified
}

pub enum MaxOccurs {
    Bounded(usize),
    Unbounded,
}

impl Default for FormChoice {
    fn default() -> Self {
        FormChoice::Unqualified
    }
}
