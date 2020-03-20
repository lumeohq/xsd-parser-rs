use crate::generator::default::{
    default_format_comment, default_format_name, default_format_type, default_modify_type,
};
use crate::parser::types::{TypeModifier, RsFile, RsEntity};
use roxmltree::Namespace;
use std::borrow::Cow;
use crate::generator::Generator2;
use crate::generator::utils::match_built_in_type;

pub trait BaseGenerator {
    fn indent(&self) -> String {
        " ".repeat(self.indent_size())
    }

    fn indent_size(&self) -> usize { 4 }

    fn format_type_name(&self, type_name: &str, gen: &Generator2) -> Cow<'_, str> {
        if let Some(t) = match_built_in_type(type_name) {
            return t.into();
        }
        default_format_type(type_name, &*gen.target_ns.borrow())
    }

    fn format_name(&self, name: &str) -> Cow<'_, str> {
        Cow::Owned(default_format_name(name))
    }

    fn format_comment(&self, comment: Option<&str>, indent: usize) -> String {
        default_format_comment(comment, 80, indent)
    }

    fn modify_type(&self, type_name: &str, modifiers: &[TypeModifier]) -> Cow<'_, str> {
        default_modify_type(type_name, modifiers)
    }

    fn join_subtypes(&self, subtypes: &[RsEntity], gen: &Generator2) -> String {
        subtypes
            .iter()
            .map(|f| gen.generate(f))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub struct DefaultBaseGenerator;
impl BaseGenerator for DefaultBaseGenerator{}
