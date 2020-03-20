use crate::generator::base::BaseGenerator;
use crate::generator::validator::gen_validate_impl;
use crate::generator::Generator2;
use crate::parser::types::{Enum, RsEntity, EnumSource};
use std::borrow::Cow;

pub trait EnumGenerator {
    fn generate(&self, entity: &Enum, gen: &Generator2) -> String {
        let name = self.get_name(entity, gen);
        let default_case = format!(
            "impl Default for {name} {{\n\
            {indent}fn default() -> {name} {{\n\
            {indent}{indent}Self::__Unknown__(\"No valid variants\".into())\n\
            {indent}}}\n\
            }}",
            name = name,
            indent = gen.base().indent()
        );

        format!(
            "{comment}{macros}\n\
            pub enum {name} {{\n\
                {cases}\n\
                {indent}__Unknown__({typename}),\n\
            }}\n\n\
            {default}\n\n\
            {validation}\n\n\
            {subtypes}\n\n",
            indent = gen.base().indent(),
            comment = self.format_comment(entity, gen),
            macros = self.macros(entity, gen),
            name = name,
            cases = self.cases(entity, gen),
            typename = self.get_type_name(entity, gen),
            default = default_case,
            subtypes = self.subtypes(entity, gen),
            validation = self.validation(entity, gen),
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

    fn subtypes(&self, entity: &Enum, gen: &Generator2) -> String {
        gen.base().join_subtypes(entity.subtypes.as_ref(), gen)
    }

    fn get_type_name(&self, entity: &Enum, gen: &Generator2) -> String {
        gen.base().format_type_name(entity.type_name.as_str(), gen).into()
    }

    fn get_name(&self, entity: &Enum, gen: &Generator2) -> String {
        gen.base().format_type_name(entity.name.as_str(), gen).into()
    }

    fn macros(&self, entity: &Enum, _gen: &Generator2) -> Cow<'static, str> {
        if entity.source == EnumSource::Union {
            return "#[derive(PartialEq, Debug, UtilsUnionSerDe)]".into();
        }
        "#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]".into()
    }

    fn format_comment(&self, entity: &Enum, gen: &Generator2) -> String {
        gen.base().format_comment(entity.comment.as_deref(), 0)
    }

    fn validation(&self, entity: &Enum, gen: &Generator2) -> Cow<'static, str> {
        // Empty validation
        Cow::Owned(gen_validate_impl(self.get_name(entity, gen).as_str(), ""))
    }
}

pub struct DefaultEnumGen;
impl EnumGenerator for DefaultEnumGen {}
