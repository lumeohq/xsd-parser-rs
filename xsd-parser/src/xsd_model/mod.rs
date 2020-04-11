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
