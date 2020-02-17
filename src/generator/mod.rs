pub mod default;
pub mod utils;

use std::borrow::Cow;
use crate::parser::types::{TupleStruct, Struct, Enum, Alias, StructField, EnumCase, Import};
use crate::generator::utils::{default_format_comment, default_format_type_name};
use inflector::cases::pascalcase::to_pascal_case;

pub trait Generator {
    fn tuple_struct_macro(&self, ts: &TupleStruct) ->  Cow<'static, str> { "".into() }
    fn struct_macro(&self, st: &Struct) -> Cow<'static, str> { "".into() }
    fn enum_macro(&self, en: &Enum) -> Cow<'static, str> { "".into() }
    fn alias_macro(&self, al: &Alias) -> Cow<'static, str> { "".into() }
    fn struct_field_macro(&self, sf: &StructField) -> Cow<'static, str> { "".into() }
    fn enum_case_macro(&self, ec: &EnumCase) -> Cow<'static, str> { "".into() }

    fn get_tuple_struct(&self, ts: &TupleStruct) ->  String {
        format!(
            "{comment}{macros}pub struct {name} (pub {typename});\n{subtypes}",
            comment = default_format_comment(ts.comment.as_deref()),
            macros = self.tuple_struct_macro(ts),
            name = ts.name,
            typename = ts.type_name,
            subtypes = ts
                .subtypes
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }

    fn get_struct(&self, st: &Struct) -> String {
        format!(
            "{comment}{macros}pub struct {name} {{\n{fields}\n}}\n{subtypes}\n{fields_subtypes}",
            comment = default_format_comment(st.comment.as_deref()),
            macros = self.struct_macro(st),
            name = st.name,
            fields = st
                .fields
                .borrow()
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
            subtypes = st
                .subtypes
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
            fields_subtypes = st
                .fields
                .borrow()
                .iter()
                .map(|f| f
                    .subtypes
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"))
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }

    fn get_enum(&self, en: &Enum) -> String {
        format!(
            "{comment}\
            {macros}\n\
            pub enum {name} \
            {{\n{cases}  \n\n\
            __Unknown__({typename})\n\
            }}\n\n\
            {default}\n\n\
            {subtypes}",
            comment = default_format_comment(en.comment.as_deref()),
            macros = self.enum_macro(en),
            name = en.name,
            cases = en
                .cases
                .iter()
                .map(|case| case.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            typename = en.type_name,
            default = format!("impl Default for {name} {{\n  fn default() -> {name} {{\n    Self::__Unknown__(\"No valid variants\".into())\n  }}\n}}",
                              name = en.name
            ),
            subtypes = en
                .subtypes
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }

    fn get_enum_case(&self, ec: &EnumCase) -> String {
        let name = self.format_type_name(ec.name.as_str());
        let comment = self.format_comment(ec.comment.as_deref());
        match &ec.type_name {
            Some(typename) => format!(
                "{comment}  {name}({typename}),",
                name = name,
                typename = typename,
                comment = comment,
            ),
            None => format!(
                "{comment}  {name},",
                name = name,
                comment = comment
            ),
        }
    }


    fn get_alias(&self, al: &Alias) -> String {
        format!(
            "//{comment} pub type {name} = {original};",
            comment = self.format_comment(al.comment.as_deref()),
            name = al.name,
            original = al.original
        )
    }

    fn get_import(&self, im: &Import) -> String {
        format!("//use {}  {};\n", im.location, im.name)
    }

    fn format_comment(&self, comment: Option<&str>) -> String {
        default_format_comment(comment)
    }

    fn format_type_name(&self, name: &str) -> String {
        default_format_type_name(name)
    }
}

