use crate::{
    generator::{utils::split_name, Generator},
    parser::types::{EnumCase, EnumSource},
};

use super::utils::{filter_type_name, sanitize};

pub trait EnumCaseGenerator {
    fn generate(&self, entity: &EnumCase, gen: &Generator) -> String {
        let typename = if entity.type_name.is_some() {
            format!("({})", self.get_type_name(entity, gen))
        } else {
            "".into()
        };
        format!(
            "{comment}{macros}{indent}{name}{typename},",
            indent = gen.base().indent(),
            name = self.get_name(entity, gen),
            comment = self.format_comment(entity, gen),
            macros = self.macros(entity, gen),
            typename = typename
        )
    }

    fn get_name(&self, entity: &EnumCase, _: &Generator) -> String {
        sanitize(filter_type_name(entity.name.split(':').last().unwrap_or(&entity.name.to_owned())))
    }

    fn get_type_name(&self, entity: &EnumCase, gen: &Generator) -> String {
        let formatted_type = gen.base().format_type_name(entity.type_name.as_ref().unwrap(), gen);
        gen.base().modify_type(formatted_type.as_ref(), &entity.type_modifiers).into()
    }

    fn format_comment(&self, entity: &EnumCase, gen: &Generator) -> String {
        gen.base().format_comment(entity.comment.as_deref(), gen.base().indent_size())
    }

    fn macros(&self, entity: &EnumCase, gen: &Generator) -> String {
        if entity.source == EnumSource::Union {
            return "".into();
        }

        let (prefix, field_name) = split_name(entity.name.as_str());
        match prefix {
            Some(p) => format!(
                "{indent}#[yaserde(prefix = \"{prefix}\", rename = \"{rename}\")]\n",
                indent = gen.base().indent(),
                prefix = p,
                rename = field_name
            ),
            None => {
                if field_name == self.get_name(entity, gen) {
                    "".into()
                } else {
                    format!(
                        "{indent}#[yaserde(rename = \"{rename}\")]\n",
                        indent = gen.base().indent(),
                        rename = field_name
                    )
                }
            }
        }
    }
}

pub struct DefaultEnumCaseGen;
impl EnumCaseGenerator for DefaultEnumCaseGen {}
