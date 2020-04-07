use crate::generator::default::{
    yaserde_for_attribute, yaserde_for_element, yaserde_for_flatten_element,
};
use crate::generator::Generator;
use crate::parser::types::{StructField, StructFieldSource, TypeModifier};

pub trait StructFieldGenerator {
    fn generate(&self, entity: &StructField, gen: &Generator) -> String {
        if entity.type_modifiers.contains(&TypeModifier::Empty) {
            return "".into();
        }
        format!(
            "{comment}{macros}{indent}pub {name}: {typename},",
            comment = self.format_comment(entity, gen),
            macros = self.macros(entity, gen),
            indent = gen.base().indent(),
            name = self.get_name(entity, gen),
            typename = self.get_type_name(entity, gen),
        )
    }

    fn get_type_name(&self, entity: &StructField, gen: &Generator) -> String {
        gen.base()
            .modify_type(
                gen.base()
                    .format_type_name(entity.type_name.as_str(), gen)
                    .as_ref(),
                &entity.type_modifiers,
            )
            .into()
    }

    fn get_name(&self, entity: &StructField, gen: &Generator) -> String {
        gen.base().format_name(entity.name.as_str()).into()
    }

    fn format_comment(&self, entity: &StructField, gen: &Generator) -> String {
        gen.base()
            .format_comment(entity.comment.as_deref(), gen.base().indent_size())
    }

    fn macros(&self, entity: &StructField, gen: &Generator) -> String {
        let indent = gen.base().indent();
        match entity.source {
            StructFieldSource::Choice => yaserde_for_flatten_element(indent.as_str()),
            StructFieldSource::Attribute => {
                yaserde_for_attribute(entity.name.as_str(), indent.as_str())
            }
            StructFieldSource::Element => yaserde_for_element(
                entity.name.as_str(),
                gen.target_ns.borrow().as_ref(),
                indent.as_str(),
            ),
            _ => "".into(),
        }
    }
}

pub struct DefaultStructFieldGen;
impl StructFieldGenerator for DefaultStructFieldGen {}
