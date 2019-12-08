use crate::xsd2::utils::{MaxOccurs, MinOccurs};

pub struct Sequence<'a, 'input> {
    pub node: roxmltree::Node<'a, 'input>,
}

impl<'a, 'input: 'a> Sequence<'a, 'input> {

    pub fn max_occurs(&self) -> Option<MaxOccurs> {
        match self.node.attribute("MaxOccurs") {
            Some(v) => match v {
                "unbounded" => Some(MaxOccurs::Unbounded),
                v => v.parse::<usize>().ok().map(|val| MaxOccurs::Bounded(val))
            },
            None => None
        }
    }

    pub fn min_occurs(&self) -> Option<MinOccurs> {
        self.node.attribute("MinOccurs").map(|v| v.parse::<usize>().ok()).flatten()
    }
}

