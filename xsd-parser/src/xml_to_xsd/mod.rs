pub mod schema;
pub mod include;
pub mod annotation;
pub mod documentation;
pub mod app_info;
pub mod import;
pub mod list;
pub mod simple_type;

use crate::xsd_model::elements::{xsd_element_type, ElementType};
use roxmltree::Attribute;
use crate::xsd_model::simple_types::any_uri::AnyUri;
use crate::xsd_model::simple_types::language::Language;
use crate::xsd_model::simple_types::id::Id;


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
//
// impl PartialEq<&str> for AnyUri<'_> {
//     fn eq(&self, other: &&str) -> bool {
//         &self.0 == other
//     }
// }


macro_rules! impl_from_attr {
    ($type_name:ident) => {
        impl<'a> From<&'a Attribute<'a>> for $type_name<'a> {
            fn from(a: &'a Attribute<'a>) -> Self {
                $type_name(a.value())
            }
        }
    }
}

impl_from_attr!(AnyUri);
impl_from_attr!(Language);
impl_from_attr!(Id);