use crate::xsd2::attribute::Attribute;
use crate::xsd2::sequence::Sequence;
use crate::xsd2::utils::find_child;

pub struct Extension<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> Extension<'a, 'input> {
    pub fn sequence(&self) -> Option<Sequence<'a, 'input>> {
        find_child(&self.node, "sequence").map(|node| Sequence{node})
    }

    pub fn has_any_attribute(&self) -> bool {
        find_child(&self.node, "anyAttribute").is_some()
    }

    pub fn base(&self) -> &'a str {
        self.node.attribute("base").expect("base required attribute for extension")
    }

    pub fn attributes(&self) -> Vec<Attribute> {
        self.node.children().
            filter(|e| e.is_element() && e.tag_name().name() == "attribute").
            map(|e| Attribute { node: e.clone() }).collect()
    }
}
