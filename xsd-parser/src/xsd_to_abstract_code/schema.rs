use crate::xsd_model::{Schema, Annotation};
use crate::abstract_code_model::module::Module;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::Entity;
use crate::xsd_model::groups::schema_top::SchemaTop;


impl<'a> Schema<'a> {
    pub fn to_module(&self, name: &'a str) -> Module {
        let mut entities = vec![];
        // TODO: includes
        // TODO: imports
        // TODO: redefines

        entities.append(&mut annotations_to_comments(self.annotations.as_slice()));

        for (schema_top, annotations) in self.content.as_slice() {
            entities.append(&mut annotations_to_comments(annotations.as_slice()));

        }



        Module {
            name,
            entities: vec![],
            private: false
        }
    }


}

fn annotations_to_comments<'a>(annotations: &[Annotation<'a>]) -> Vec<Entity<'a>> {
    annotations
        .iter()
        .map(Comment::from_annotation)
        .map(Entity::Comment)
        .collect()
}

fn schema_top_to_entity(schema_top: &SchemaTop) {
    match schema_top {
        SchemaTop::SimpleType(value) => {}
        SchemaTop::ComplexType(value) => {}
        SchemaTop::Group(value) => {}
        SchemaTop::AttributeGroup(value) => {}
        SchemaTop::Element(value) => {}
        SchemaTop::Attribute(value) => {}
        SchemaTop::Notation(value) => {}
    };
}