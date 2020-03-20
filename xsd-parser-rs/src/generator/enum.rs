use crate::generator::base::BaseGenerator;
use crate::generator::validator::gen_validate_impl;
use crate::generator::Generator2;
use crate::parser::types::{Enum, RsEntity};
use std::borrow::Cow;

pub trait EnumGenerator {
    fn generate(&self, entity: &Enum, gen: &Generator2) -> String {
        let base = gen.base();
        let name = self.get_type_name(entity, base);

        format!(
            "{comment}{macros}\n\
            pub enum {name} {{\n\
                {cases}\n\
                {indent}__Unknown__({typename}),\n\
            }}\n\n\
            {default}\n\n\
            {validation}\n\n\
            {subtypes}\n\n
            ",
            indent = base.indent(),
            comment = self.format_comment(entity.comment.as_deref(), base),
            macros = self.macros(entity),
            name = name,
            cases = self.cases(entity, gen),
            typename = self.get_type_name(entity, base),
            default = self.impl_default(name.as_str(), base),
            subtypes = self.subtypes(entity.subtypes.as_ref(), gen),
            validation = self.validation(entity, name.as_ref()),
        )
    }

    fn impl_default(&self, name: &str, base: &Box<dyn BaseGenerator>) -> String {
        format!(
            "impl Default for {name} {{\n{indent}fn default() -> {name} {{\n{indent}{indent}Self::__Unknown__(\"No valid variants\".into())\n{indent}}}\n}}",
            name = name,
            indent = base.indent()
        )
    }

    fn cases(&self, entity: &Enum, gen: &Generator2) -> String {
        entity
            .cases
            .iter()
            .map(|case| gen.enum_case_gen().generate(case, gen))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn subtypes(&self, subtypes: &[RsEntity], gen: &Generator2) -> String {
        subtypes
            .iter()
            .map(|f| gen.generate(f))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn get_type_name(&self, entity: &Enum, base: &Box<dyn BaseGenerator>) -> String {
        base.format_type_name(entity.name.as_str()).into()
    }

    fn macros(&self, _: &Enum) -> Cow<'static, str> {
        "".into()
    }

    fn format_comment(&self, comment: Option<&str>, base: &Box<dyn BaseGenerator>) -> String {
        base.format_comment(comment, base.indent_size())
    }

    fn validation(&self, _entity: &Enum, formatted_type_name: &str) -> Cow<'static, str> {
        // Empty validation
        Cow::Owned(gen_validate_impl(formatted_type_name, ""))
    }
}

pub struct DefaultEnumGen;
impl EnumGenerator for DefaultEnumGen {}
