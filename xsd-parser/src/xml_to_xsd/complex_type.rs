use roxmltree::Node;
use crate::xsd_model::TopLevelComplexType;

impl<'a> TopLevelComplexType<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {

    }
}