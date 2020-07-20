use crate::xsd_model::TopLevelSimpleType;
use crate::abstract_code_model::structure::Structure;
use std::borrow::Cow;
use crate::abstract_code_model::comment::Comment;
use crate::xsd_model::groups::simple_derivation::SimpleDerivation;
use crate::abstract_code_model::Entity;

impl<'a> TopLevelSimpleType<'a> {
    pub fn to_struct(&self) -> Structure<'a> {
        Structure {
            name: Some(Cow::from(self.name.0)),
            comment: Comment::from_opt_annotation(&self.annotation),
            fields: vec![],
            subtypes: vec![],
        }
    }
}

