use crate::xsd2::node_types::Element;
use crate::xsd2::utils::find_child;
use crate::xsd2::node_types::Choice as ChoiceType;
use crate::xsd2::node_types::AnyElement as AnyElementType;
use crate::xsd2::node_types::Extension as ExtensionType;
use crate::xsd2::node_types::Sequence as SequenceType;
use crate::xsd2::node_types::Attribute;

pub trait Node {
    fn node(&self) -> & roxmltree::Node<'_, '_>;
}

pub type MinOccurs = usize;
pub enum MaxOccurs {
    Bounded(usize),
    Unbounded,
    None
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

pub trait Attributes: Node {
    fn attributes(&self) -> Vec<Attribute> {
        self.node().children()
            .filter(|e| e.is_element() && e.tag_name().name() == "attribute")
            .map(|e| Attribute { node: e.clone() }).collect()
    }
}

pub trait Choice: Node {
    fn choice(&self) -> Option<ChoiceType> {
        find_child(self.node(), "choice").map(|n| ChoiceType{node: n})
    }
}

pub trait Documentation: Node {
    fn documentation(&self) -> Option<&str> {
        find_child(self.node(), "annotation").
            and_then(|node| find_child(&node, "documentation")).
            and_then(|node| node.text())
    }
}

pub trait AnyElement: Node {
    fn any_element(&self) -> Option<AnyElementType> {
        find_child(self.node(), "any").map(|n| AnyElementType{node: n})
    }
}

pub trait Extension: Node {
    fn extension(&self) -> Option<ExtensionType> {
        find_child(self.node(), "extension").map(|n| ExtensionType{node: n})
    }
}

pub trait Sequence: Node {
    fn sequence(&self) -> Option<SequenceType> {
        find_child(self.node(), "sequence").map(|n| SequenceType{node: n})
    }
}

pub trait Name: Node {
    fn name(&self) -> Option<&str> {
        self.node().attribute("name")
    }
}

pub trait TypeName: Node {
    fn type_name(&self) -> Option<&str> {
        self.node().attribute("type")
    }

    fn type_prefix(&self) -> Option<&str> {
        match self.type_name() {
            Some(tn) => {
                let prefix = tn
                    .split(':')
                    .next()
                    .unwrap_or("");
                if prefix.len() > 0 {
                    Some(prefix)
                }
                else {
                    None
                }
            },
            None => None
        }
    }
}
