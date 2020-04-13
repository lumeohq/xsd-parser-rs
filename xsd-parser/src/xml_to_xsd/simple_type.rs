use crate::xsd_model::{LocalSimpleType, List, Union};
use roxmltree::Node;
use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::Annotation;
use crate::xsd_model::groups::simple_derivation::SimpleDerivation;

impl<'a> LocalSimpleType<'a> {
     pub fn parse(node: Node<'a, '_>) -> Result<LocalSimpleType<'a>, String> {
         let mut annotation = None;
         let mut id = None;
         let mut attributes = vec![];
         for attr in node.attributes() {
             match attr.name() {
                 "id" => id = Some(attr.into()),
                 _ => attributes.push(attr.clone())
             }
         }

         for ch in node.children().filter(|n| n.is_element()) {
             match  ch.xsd_type()? {
                 ElementType::Annotation => {annotation = Some(Annotation::parse(ch)?)}
                 x => return Ok(Self {
                     annotation,
                     content_choice: Box::new(SimpleDerivation::parse(ch,x)?),
                     id,
                     attributes
                 })
            };
         }


         Err("".to_string())
     }
}

impl<'a> SimpleDerivation<'a> {
    pub fn parse(node: Node<'a, '_>, element_type: ElementType) -> Result<SimpleDerivation<'a>, String> {
        let res = match element_type {
            ElementType::Union => Self::Union(Union::parse(node)?),
            // ElementType::Restriction => Self::Restriction(Restriction::parse(node)?),
            ElementType::List => Self::List(List::parse(node)?),
            _ => return Err(format!("Invalid simple derivation content: {:?}", node))
        };

        Ok(res)
    }
}

