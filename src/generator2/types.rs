use crate::generator2::utils::{get_field_comment, get_structure_comment};
use core::fmt;

pub struct Struct {
    pub name: String,
    pub comment: Option<String>,
    pub fields: Vec<StructField>,
    pub macros: String,
    pub subtypes: Vec<RsType>,
}

impl fmt::Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{comment}{macros}pub struct {name} {{\n{fields}\n}}\n{subtypes}",
            comment = get_structure_comment(self.comment.as_deref()),
            macros = self.macros,
            name = self.name,
            fields = self
                .fields
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            subtypes = self
                .subtypes
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

pub struct StructField {
    pub name: String,
    pub type_name: String,
    pub comment: Option<String>,
    pub macros: String,
}

impl fmt::Display for StructField {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{macros}  pub {name}: {typename},  {comment}",
            macros = self.macros,
            name = self.name,
            typename = self.type_name,
            comment = get_field_comment(self.comment.as_deref())
        )
    }
}

pub struct TupleStruct {
    pub name: String,
    pub comment: Option<String>,
    pub type_name: String,
    pub macros: String,
    pub subtypes: Vec<RsType>,
}

impl fmt::Display for TupleStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{comment}{macros}pub struct {name} (pub {typename});\n{subtypes}",
            comment = get_structure_comment(self.comment.as_deref()),
            macros = self.macros,
            name = self.name,
            typename = self.type_name,
            subtypes = self
                .subtypes
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

pub struct Enum {
    pub name: String,
    pub cases: Vec<EnumCase>,
    pub comment: Option<String>,
    pub type_name: String,
}

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{comment}pub enum {name} {{\n{cases}  \n__Unknown__({typename})\n}}\n",
            comment = get_structure_comment(self.comment.as_deref()),
            name = self.name,
            cases = self
                .cases
                .iter()
                .map(|case| case.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            typename = self.type_name
        )
    }
}

pub struct EnumCase {
    pub name: String,
    pub comment: Option<String>,
    pub value: String,
    pub type_name: Option<String>,
}

impl fmt::Display for EnumCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.type_name {
            Some(tn) => write!(
                f,
                "  {name}({typename}),  {comment}",
                name = self.name,
                typename = tn,
                comment = get_field_comment(self.comment.as_deref()),
            ),
            None => write!(
                f,
                "  {name},  {comment}",
                name = self.name,
                comment = get_field_comment(self.comment.as_deref())
            ),
        }
    }
}

pub struct Alias {
    pub name: String,
    pub original: String,
    pub comment: String,
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{comment}pub type {name} = {original};\n",
            comment = self.comment,
            name = self.name,
            original = self.original
        )
    }
}

pub enum RsType {
    Struct(Struct),
    TupleStruct(TupleStruct),
    Enum(Enum),
    Alias(Alias),
}

impl fmt::Display for RsType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use RsType::*;
        match self {
            Struct(s) => write!(f, "{}", s),
            TupleStruct(tp) => write!(f, "{}", tp),
            Enum(e) => write!(f, "{}", e),
            Alias(al) => write!(f, "{}", al),
        }
    }
}

impl RsType {
    pub fn name(&self) -> &str {
        use RsType::*;
        match self {
            Struct(s) => s.name.as_str(),
            TupleStruct(tp) => tp.name.as_str(),
            Enum(e) => e.name.as_str(),
            Alias(al) => al.name.as_str(),
        }
    }
}
