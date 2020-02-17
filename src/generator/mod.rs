pub mod default;

use std::borrow::Cow;
use crate::parser::types::{TupleStruct, Struct, Enum, Alias, StructField, EnumCase};

pub trait MacroGenerator {
    fn tuple_struct_macro(&self, ts: &TupleStruct) ->  Cow<'static, str> { "".into() }
    fn struct_macro(&self, st: &Struct) -> Cow<'static, str> { "".into() }
    fn enum_macro(&self, en: &Enum) -> Cow<'static, str> { "".into() }
    fn alias_macro(&self, al: &Alias) -> Cow<'static, str> { "".into() }
    fn struct_field_macro(&self, sf: &StructField) -> Cow<'static, str> { "".into() }
    fn enum_case_macro(&self, ec: &EnumCase) -> Cow<'static, str> { "".into() }
}

