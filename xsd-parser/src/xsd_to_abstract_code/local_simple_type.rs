use crate::xsd_model::LocalSimpleType;
use crate::abstract_code_model::Entity;
use crate::xsd_to_abstract_code::simple_derivation::simple_derivation_to_entity;
use crate::abstract_code_model::structure::Structure;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::enumeration::Enumeration;

impl<'a> LocalSimpleType<'a> {
    pub fn to_entity(&self) -> Entity {
        match simple_derivation_to_entity(&self.content_choice) {
            Entity::StructFields(fields) => {
                Entity::Struct(
                    Structure {
                        name: None,
                        comment: Comment::from_opt_annotation(&self.annotation),
                        fields,
                        subtypes: vec![]
                    }
                )
            },
            Entity::EnumItems(items) => {
                Entity::Enum(
                    Enumeration {
                        name: None,
                        items,
                        comment: Comment::from_opt_annotation(&self.annotation),
                        subtypes: vec![]
                    }
                )
            },
            _ => unreachable!()
        }
    }
}