use crate::generator::base::BaseGenerator;
use crate::generator::Generator2;
use crate::parser::types::Alias;

pub trait AliasGenerator {
    fn generate(&self, entity: &Alias, gen: &Generator2) -> String {
        let base = gen.base();
        format!(
            "//{comment} pub type {name} = {original};",
            comment = self.format_comment(entity.comment.as_deref(), base),
            name = self.format_name(entity.name.as_str(), base),
            original = self.format_original_type(entity.original.as_str(), base)
        )
    }

    fn format_comment(&self, comment: Option<&str>, base: &Box<dyn BaseGenerator>) -> String {
        base.format_comment(comment, base.indent_size()).into()
    }

    fn format_name(&self, name: &str, base: &Box<dyn BaseGenerator>) -> String {
        base.format_type_name(name).into()
    }

    fn format_original_type(&self, name: &str, base: &Box<dyn BaseGenerator>) -> String {
        base.format_type_name(name).into()
    }
}

pub struct DefaultAliasGen;
impl AliasGenerator for DefaultAliasGen {}
