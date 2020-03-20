use crate::generator::base::BaseGenerator;
use crate::generator::Generator2;
use crate::parser::types::EnumCase;

pub trait EnumCaseGenerator {
    fn generate(&self, entity: &EnumCase, gen: &Generator2) -> String {
        let base = gen.base();

        match &entity.type_name {
            Some(_) => format!(
                "{comment}{macros}{indent}{name}({typename}),",
                name = self.get_name(entity, base),
                typename = self.get_type_name(entity, base),
                indent = base.indent(),
                comment = self.format_comment(entity.comment.as_deref(), base),
                macros = self.macros(entity)
            ),
            None => format!(
                "{comment}{macros}{indent}{name},",
                indent = base.indent(),
                name = self.get_name(entity, base),
                comment = self.format_comment(entity.comment.as_deref(), base),
                macros = self.macros(entity)
            ),
        }
    }

    fn get_name(&self, entity: &EnumCase, base: &Box<dyn BaseGenerator>) -> String {
        base.format_type_name(entity.name.as_str())
            .split("::")
            .last()
            .unwrap()
            .to_string()
    }

    fn get_type_name(&self, entity: &EnumCase, base: &Box<dyn BaseGenerator>) -> String {
        let formatted_type = base.format_type_name(entity.type_name.as_ref().unwrap());
        base.modify_type(formatted_type.as_ref(), &entity.type_modifiers)
            .into()
    }

    fn format_comment(&self, comment: Option<&str>, base: &Box<dyn BaseGenerator>) -> String {
        base.format_comment(comment, base.indent_size()).into()
    }

    fn macros(&self, _: &EnumCase) -> String {
        "".into()
    }
}

pub struct DefaultEnumCaseGen;
impl EnumCaseGenerator for DefaultEnumCaseGen {}
