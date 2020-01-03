use crate::xsd2::utils::{Node, Elements, MinMaxOccurs};
use crate::xsd2::sequence::Element;
use crate::create_node_type;

create_node_type!(Choice, Elements, MinMaxOccurs);
