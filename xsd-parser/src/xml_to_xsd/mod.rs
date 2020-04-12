use crate::xsd_model::elements::{xsd_element_type, ElementType};

pub mod schema;

pub const XSD_NS_URI: &str = "http://www.w3.org/2001/XMLSchema";

pub trait XsdNode {
    fn xsd_type(&self) -> Result<ElementType, String>;
}

impl XsdNode for roxmltree::Node<'_, '_> {
    fn xsd_type(&self) -> Result<ElementType, String> {
        if let Some(uri) = self.tag_name().namespace() {
            if uri != XSD_NS_URI {
                Err(format!(
                    "Invalid prefix for xsd element: {:?}",
                    self.tag_name()
                ))?
            }
        }
        xsd_element_type(self.tag_name().name())
    }
}
