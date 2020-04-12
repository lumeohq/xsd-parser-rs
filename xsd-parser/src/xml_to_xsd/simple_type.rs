use crate::xsd_model::LocalSimpleType;
use roxmltree::Node;

impl<'a> LocalSimpleType<'a> {
     pub fn parse(node: Node<'a, '_>) -> Result<LocalSimpleType<'a>, String> {
         //let mut res = LocalSimpleType::default();

         Err("".to_string())
     }
}