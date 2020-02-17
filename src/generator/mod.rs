pub mod default;
pub mod utils;

use std::borrow::Cow;
use crate::parser::types::{TupleStruct, Struct, Enum, Alias, StructField, EnumCase};
use crate::generator::utils::get_formatted_comment;

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
            comment = get_formatted_comment(ts.comment.as_deref()),
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
            comment = get_formatted_comment(st.comment.as_deref()),
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
}

