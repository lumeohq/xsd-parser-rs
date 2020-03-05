pub mod default;
mod utils;
pub mod validator;

use crate::parser::types::{
    Alias, Enum, EnumCase, Import, RsEntity, Struct, StructField, TupleStruct, TypeModifier,
};

use crate::generator::default::{
    default_format_comment, default_format_name, default_format_type, default_modify_type,
};
use crate::generator::validator::{gen_facet_validation, gen_validate_impl};
use roxmltree::Namespace;
use std::borrow::Cow;

pub trait Generator<'input> {
    fn target_ns(&self) -> &Option<Namespace<'_>>;

    fn tuple_struct_macro(&self, _: &TupleStruct) -> Cow<'static, str> {
        "".into()
    }
    fn struct_macro(&self, _: &Struct) -> Cow<'static, str> {
        "".into()
    }
    fn enum_macro(&self, _: &Enum) -> Cow<'static, str> {
        "".into()
    }
    fn alias_macro(&self, _: &Alias) -> Cow<'static, str> {
        "".into()
    }
    fn struct_field_macro(&self, _: &StructField) -> Cow<'static, str> {
        "".into()
    }
    fn enum_case_macro(&self, _: &EnumCase) -> Cow<'static, str> {
        "".into()
    }

    fn gen_rs_entity(&self, entity: &RsEntity) -> String {
        use RsEntity::*;
        match entity {
            TupleStruct(tp) => self.gen_tuple_struct(tp),
            Struct(st) => self.gen_struct(st),
            Enum(en) => self.gen_enum(en),
            Import(im) => self.gen_import(im),
            Alias(al) => self.get_alias(al),
            EnumCase(ec) => self.gen_enum_case(ec),
            StructField(sf) => self.get_struct_field(sf),
        }
    }

    fn gen_tuple_struct(&self, ts: &TupleStruct) -> String {
        let typename = self.modify_type(
            self.format_type(ts.type_name.as_str()).as_ref(),
            &ts.type_modifiers,
        );
        format!(
            "{comment}{macros}pub struct {name} (pub {typename});\n{subtypes}\n{validation}\n",
            comment = self.format_comment(ts.comment.as_deref(), 0),
            macros = self.tuple_struct_macro(ts),
            name = self.format_type(ts.name.as_str()),
            typename = typename,
            subtypes = ts
                .subtypes
                .iter()
                .map(|f| self.gen_rs_entity(f))
                .collect::<Vec<String>>()
                .join("\n"),
            validation = self.gen_tuple_struct_validation(ts),
        )
    }

    fn gen_tuple_struct_validation(&self, ts: &TupleStruct) -> Cow<'static, str> {
        let body = ts
            .facets
            .iter()
            .map(|f| gen_facet_validation(&f.facet_type, "0"))
            .fold(String::new(), |x, y| (x + &y));
        Cow::Owned(gen_validate_impl(
            self.format_type(ts.name.as_str()).as_ref(),
            body.as_str(),
        ))
    }

    fn gen_struct(&self, st: &Struct) -> String {
        let name = self.format_type(st.name.as_str());
        format!(
            "{comment}{macros}pub struct {name} {{\n{fields}\n}}\n\n{validation}\n{subtypes}\n{fields_subtypes}",
            comment = self.format_comment(st.comment.as_deref(), 0),
            macros = self.struct_macro(st),
            name = name,
            fields = st
                .fields
                .borrow()
                .iter()
                .map(|f| self.get_struct_field(f))
                .collect::<Vec<String>>()
                .join("\n\n"),
            subtypes = st
                .subtypes
                .iter()
                .map(|f| self.gen_rs_entity(f))
                .collect::<Vec<String>>()
                .join("\n\n"),
            fields_subtypes = st
                .fields
                .borrow()
                .iter()
                .map(|f| f
                    .subtypes
                    .iter()
                    .map(|e| self.gen_rs_entity(e))
                    .collect::<Vec<String>>()
                    .join("\n"))
                .collect::<Vec<String>>()
                .join(""),
            validation = gen_validate_impl(
                name.as_ref(),
                ""
            ),
        )
    }

    fn gen_enum(&self, en: &Enum) -> String {
        let name = self.format_type(en.name.as_str());
        format!(
            "{comment}{macros}\npub enum {name} {{\n{cases}\n __Unknown__({typename})\n\
            }}\n\n\
            {default}\n\n\
            {validation}\n\n
            {subtypes}\n\n\
            ",
            comment = self.format_comment(en.comment.as_deref(), 0),
            macros = self.enum_macro(en),
            name = name,
            cases = en
                .cases
                .iter()
                .map(|case| self.gen_enum_case(case))
                .collect::<Vec<String>>()
                .join("\n"),
            typename = self.format_type(en.type_name.as_str()),
            default = format!("impl Default for {name} {{\n  fn default() -> {name} {{\n    Self::__Unknown__(\"No valid variants\".into())\n  }}\n}}",
                              name = name
            ),
            subtypes = en
                .subtypes
                .iter()
                .map(|f| self.gen_rs_entity(f))
                .collect::<Vec<String>>()
                .join("\n\n"),
            validation = gen_validate_impl(
                name.as_ref(),
                ""
            ),
        )
    }

    fn gen_enum_case(&self, ec: &EnumCase) -> String {
        let name = self.format_type(ec.name.as_str());
        let comment = self.format_comment(ec.comment.as_deref(), 2);
        let macros: Cow<'static, str> = if name.as_ref() == ec.name.as_str() {
            "".into()
        } else {
            self.enum_case_macro(ec)
        };
        match &ec.type_name {
            Some(typename) => format!(
                "{comment}{macros}    {name}({typename}),",
                name = name,
                typename = self.modify_type(
                    self.format_type(typename.as_str()).as_ref(),
                    &ec.type_modifiers
                ),
                comment = comment,
                macros = macros
            ),
            None => format!(
                "{comment}{macros}  {name},",
                name = name,
                comment = comment,
                macros = macros
            ),
        }
    }

    fn get_struct_field(&self, sf: &StructField) -> String {
        let typename = self.modify_type(
            self.format_type(sf.type_name.as_str()).as_ref(),
            &sf.type_modifiers,
        );
        format!(
            "{comment}{macros}    pub {name}: {typename},",
            macros = self.struct_field_macro(sf),
            name = self.format_name(sf.name.as_str()),
            typename = typename,
            comment = self.format_comment(sf.comment.as_deref(), 4)
        )
    }

    fn get_alias(&self, al: &Alias) -> String {
        format!(
            "//{comment} pub type {name} = {original};",
            comment = self.format_comment(al.comment.as_deref(), 0),
            name = self.format_type(al.name.as_str()),
            original = self.format_type(al.original.as_str())
        )
    }

    fn modify_type(&self, type_name: &str, modifiers: &[TypeModifier]) -> Cow<'_, str> {
        default_modify_type(type_name, modifiers)
    }

    fn gen_import(&self, im: &Import) -> String {
        format!("//use {}  {};\n", im.location, im.name)
    }

    fn format_comment(&self, comment: Option<&str>, indent: usize) -> String {
        default_format_comment(comment, 80, indent)
    }

    fn format_name(&self, name: &str) -> Cow<'_, str> {
        Cow::Owned(default_format_name(name))
    }

    fn format_type(&self, type_name: &str) -> Cow<'_, str> {
        default_format_type(type_name, self.target_ns())
    }
}
