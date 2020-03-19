use crate::parser::types::{StructField, TypeModifier};
use crate::generator::Generator2;
use crate::generator::base::BaseGenerator;
use std::borrow::Cow;


pub trait StructFieldGenerator {
    fn generate(&self, entity: &StructField, gen: &Generator2) -> String {
        if entity.type_modifiers.contains(&TypeModifier::Empty) {
            return "".into();
        }

        let base = gen.base();
        let typename = self.get_type_name(entity, base);
        format!(
            "{comment}{macros}{indent}pub {name}: {typename},",
            indent = base.indent(),
            macros = self.macros(entity),
            name = self.get_name(entity, base),
            typename = typename,
            comment = self.format_comment(entity.comment.as_deref(), base)
        )
    }

    fn get_type_name(&self, entity: &StructField, base: &Box<dyn BaseGenerator>) -> String {
        base.modify_type(
            base.format_type_name(entity.type_name.as_str()).as_ref(),
            &entity.type_modifiers
        ).into()
    }

    fn get_name(&self, entity: &StructField, base: &Box<dyn BaseGenerator>) -> String {
        base.format_name(entity.name.as_str()).into()
    }

    fn format_comment(&self, comment: Option<&str>, base: &Box<dyn BaseGenerator>) -> String {
        base.format_comment(comment, base.indent_size()).into()
    }

    fn macros(&self, _: &StructField) -> String { "".into() }
}

pub struct DefaultStructFieldGen;
impl StructFieldGenerator for DefaultStructFieldGen{}