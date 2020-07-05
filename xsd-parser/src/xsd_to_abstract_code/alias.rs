use crate::abstract_code_model::alias::Alias;
use crate::xsd_model::{TopLevelElement, TopLevelAttribute};
use std::borrow::Cow;
use crate::abstract_code_model::TypeName;
use crate::abstract_code_model::comment::Comment;

impl<'a> Alias<'a>{
    pub fn from_top_level_element(element: &'a TopLevelElement) -> Option<Alias<'a>> {
        element.type_.as_ref().map(|ty| Alias{
                name: Cow::from(element.name.0),
                type_name: TypeName::from_qname(ty),
                comment: Comment::from_opt_annotation(&element.annotation)
            }
        )

    }

    pub fn from_top_level_attribute(attribute: &'a TopLevelAttribute) -> Option<Alias<'a>> {
        attribute.type_.as_ref().map(|ty| Alias{
                name: Cow::from(attribute.name.0),
                type_name: TypeName::from_qname(ty),
                comment: Comment::from_opt_annotation(&attribute.annotation)
        })
    }
}