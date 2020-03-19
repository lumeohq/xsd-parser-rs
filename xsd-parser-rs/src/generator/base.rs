use crate::generator::default::{
    default_format_comment, default_format_name, default_format_type, default_modify_type,
};
use crate::parser::types::TypeModifier;
use roxmltree::Namespace;
use std::borrow::Cow;

pub trait BaseGenerator {
    fn target_ns(&self) -> &Option<Namespace<'_>>;

    fn indent(&self) -> String {
        " ".repeat(self.indent_size())
    }

    fn indent_size(&self) -> usize {
        4
    }

    fn format_type_name(&self, type_name: &str) -> Cow<'_, str> {
        default_format_type(type_name, self.target_ns())
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
}

pub struct DefaultBaseGenerator<'input> {
    pub target_ns: Option<Namespace<'input>>,
}

impl BaseGenerator for DefaultBaseGenerator<'_> {
    fn target_ns(&self) -> &Option<Namespace<'_>> {
        &self.target_ns
    }
}
