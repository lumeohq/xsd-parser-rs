use crate::xsd_model::{List, LocalSimpleType};
use crate::abstract_code_model::structure::{StructField, TypeModifier};
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::TypeName;

impl<'a> List<'a> {
    pub fn to_struct_field(&self) -> StructField {
        let entities = match &self.simple_type {
            Some(val) => vec![val.to_entity()],
            None => vec![]
        };

        StructField {
            name: None,
            type_name: self.item_type.as_ref().map(TypeName::from_qname),
            comment: Comment::from_opt_annotation(&self.annotation),
            entities,
            modifiers: vec![TypeModifier::Array]
        }
    }
}