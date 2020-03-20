use crate::generator::base::BaseGenerator;
use crate::generator::Generator2;
use crate::parser::types::Alias;

pub trait AliasGenerator {
    fn generate(&self, entity: &Alias, gen: &Generator2) -> String {
        format!(
            "//{comment} pub type {name} = {original};\n",
            comment = self.format_comment(entity.comment.as_deref(), gen),
            name = self.format_name(entity.name.as_str(), gen),
            original = self.format_original_type(entity.original.as_str(), gen)
        )
    }

    fn format_comment(&self, comment: Option<&str>, gen: &Generator2) -> String {
        gen.base().format_comment(comment, 0).into()
    }

    fn format_name(&self, name: &str, gen: &Generator2) -> String {
        gen.base().format_type_name(name, gen).into()
    }

    fn format_original_type(&self, name: &str, gen: &Generator2) -> String {
        gen.base().format_type_name(name, gen).into()
    }
}

pub struct DefaultAliasGen;
impl AliasGenerator for DefaultAliasGen {}
