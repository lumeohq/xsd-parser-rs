pub mod default;
mod utils;
pub mod validator;

pub mod tuple_struct;
pub mod r#struct;
pub mod base;
pub mod struct_field;
pub mod enum_case;

use crate::parser::types::{Alias, Enum, EnumCase, Import, RsEntity, Struct, StructField, TupleStruct, TypeModifier, RsFile};

use crate::generator::default::{
    default_format_comment, default_format_enum_case_name, default_format_name,
    default_format_type, default_modify_type,
};
use crate::generator::validator::{gen_facet_validation, gen_validate_impl};
use roxmltree::Namespace;
use std::borrow::Cow;
use crate::generator::tuple_struct::{TupleStructGenerator, DefaultTupleStructGen};
use crate::generator::base::{BaseGenerator, DefaultBaseGenerator};
use crate::generator::r#struct::{StructGen, DefaultStructGen };
use crate::generator::struct_field::{StructFieldGenerator, DefaultStructFieldGen};


pub trait Generator {
    fn target_ns(&self) -> &Option<Namespace<'_>>;
    fn indent(&self) -> &str { "    " }

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
            Alias(al) => self.gen_alias(al),
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
        let fields = st
            .fields
            .borrow()
            .iter()
            .map(|f| self.get_struct_field(f))
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join("\n\n");

        let name = self.format_type(st.name.as_str());

        format!(
            "{comment}{macros}pub struct {name} {{{fields}}}\n\n{validation}\n{subtypes}\n{fields_subtypes}",
            comment = self.format_comment(st.comment.as_deref(), 0),
            macros = self.struct_macro(st),
            name = name,
            fields = if fields.is_empty() {
                fields
            } else {
                format!("\n{}\n", fields)
            },
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
            "{comment}{macros}\npub enum {name} {{\n{cases}\n{indent}__Unknown__({typename}),\n\
            }}\n\n\
            {default}\n\n\
            {validation}\n\n\
            {subtypes}\n\n
            ",
            indent = self.indent(),
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
            default = format!(
                "impl Default for {name} {{\n{indent}fn default() -> {name} {{\n{indent}{indent}Self::__Unknown__(\"No valid variants\".into())\n{indent}}}\n}}",
                name = name,
                indent = self.indent()
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
        let name = self.get_enum_case_name(ec);
        let comment = self.format_comment(ec.comment.as_deref(), 4);
        let macros: Cow<'static, str> = if name == ec.name {
            "".into()
        } else {
            // if rename required
            self.enum_case_macro(ec)
        };
        match &ec.type_name {
            Some(typename) => format!(
                "{comment}{macros}{indent}{name}({typename}),",
                name = name,
                typename = self.modify_type(
                    self.format_type(typename.as_str()).as_ref(),
                    &ec.type_modifiers
                ),
                indent = self.indent(),
                comment = comment,
                macros = macros
            ),
            None => format!(
                "{comment}{macros}{indent}{name},",
                indent = self.indent(),
                name = name,
                comment = comment,
                macros = macros
            ),
        }
    }

    fn get_enum_case_name(&self, ec: &EnumCase) -> String {
        self.format_enum_case_name(ec.name.as_str())
            .split("::")
            .last()
            .unwrap()
            .to_string()
    }

    fn get_struct_field(&self, sf: &StructField) -> String {
        if sf.type_modifiers.contains(&TypeModifier::Empty) {
            return "".into();
        }
        let typename = self.modify_type(
            self.format_type(sf.type_name.as_str()).as_ref(),
            &sf.type_modifiers,
        );
        format!(
            "{comment}{macros}{indent}pub {name}: {typename},",
            indent = self.indent(),
            macros = self.struct_field_macro(sf),
            name = self.format_name(sf.name.as_str()),
            typename = typename,
            comment = self.format_comment(sf.comment.as_deref(), 4)
        )
    }

    fn gen_alias(&self, al: &Alias) -> String {
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

    fn format_enum_case_name(&self, name: &str) -> Cow<'_, str> {
        default_format_enum_case_name(name, self.target_ns())
    }
}











#[derive(Default)]
pub struct Generator2<'input> {
    target_ns: Option<Namespace<'input>>,

    pub tsg: Option<Box<dyn TupleStructGenerator>>,
    pub sg: Option<Box<dyn StructGen>>,
    pub struct_field_gen: Option<Box<dyn StructFieldGenerator>>,
    pub base: Option<Box<dyn BaseGenerator>>
}

impl<'input> Generator2<'input> {
    pub fn new(schema: &RsFile<'input>) -> GeneratorBuilder<'input>{
        GeneratorBuilder {
            gen: Self {
                target_ns: schema.target_ns.clone(),
                ..Default::default()
            }
        }
    }
    pub fn generate(&self, entity: &RsEntity) -> String {
        match entity {
            RsEntity::TupleStruct(ts) => self.tsg.as_ref().unwrap().generate(ts, self),
            RsEntity::Struct(st) => self.sg.as_ref().unwrap().generate(st, self),
            RsEntity::StructField(sf) => self.struct_field_gen.as_ref().unwrap().generate(sf, self),
            _ => unreachable!()
        }
    }

    pub fn base(&self) -> &Box<dyn BaseGenerator> {
        self.base.as_ref().unwrap()
    }

    pub fn struct_field_gen(&self) ->  &Box<dyn StructFieldGenerator> {
        self.struct_field_gen.as_ref().unwrap()
    }
}

#[derive(Default)]
pub struct GeneratorBuilder<'input>{
    gen: Generator2<'input>
}

impl<'input> GeneratorBuilder<'input> {
    pub fn with_base_gen(mut self, base: Box<dyn BaseGenerator>) -> Self {
        self.gen.base = Some(base);
        self
    }

    pub fn with_tuple_struct_gen(mut self, tsg: Box<dyn TupleStructGenerator>) -> Self {
        self.gen.tsg = Some(tsg);
        self
    }

    pub fn with_struct_gen(mut self, sg: Box<dyn StructGen>) -> Self {
        self.gen.sg = Some(sg);
        self
    }

    pub fn build(mut self) -> Generator2<'input> {
        let mut gen = self.gen;
        gen.base.get_or_insert_with(
            || Box::new(DefaultBaseGenerator{target_ns: None})
        );  //.set_target_ns(&gen.target_ns);

        gen.tsg.get_or_insert_with(
            || Box::new(DefaultTupleStructGen{})
        );

        gen.sg.get_or_insert_with(
            || Box::new(DefaultStructGen{})
        );

        gen.struct_field_gen.get_or_insert_with(
            || Box::new(DefaultStructFieldGen{})
        );
        gen
    }
}













#[cfg(test)]
mod test {
    use crate::parser::types::{TupleStruct, RsEntity, Struct};
    use std::borrow::{Cow};
    use crate::generator::default::default_format_type;
    use crate::generator::Generator2;


    #[test]
    fn foo() {
    }
}
