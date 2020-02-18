use core::fmt;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::parser::constants::tag;
use crate::parser::utils::{get_formatted_comment, get_type_name};
use roxmltree::Namespace;

#[derive(Debug, Clone)]
pub struct File<'input> {
    pub name: String,
    pub namespace: Option<String>,
    pub types: Vec<RsEntity>,
    pub target_ns: Option<Namespace<'input>>
}

impl fmt::Display for File<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "//generated file\n{types}",
            types = self
                .types
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct Struct {
    pub name: String,
    pub comment: Option<String>,
    pub fields: RefCell<Vec<StructField>>,
    pub subtypes: Vec<RsEntity>,
}

impl Struct {
    pub fn get_types_map(&self) -> HashMap<&String, &Self> {
        let mut map = HashMap::new();
        map.insert(&self.name, self);
        for ty in &self.subtypes {
            if let RsEntity::Struct(st) = ty {
                map.extend(st.get_types_map());
            }
        }
        map
    }

    pub fn extend_base(&self, types: &HashMap<&String, &Self>) {
        let mut fields = self
            .fields
            .borrow()
            .iter()
            .filter(|f| f.name.as_str() == tag::BASE)
            .flat_map(|f| {
                types
                    .get(&f.type_name)
                    .map(|s| s.fields.borrow().clone())
                    .unwrap_or_else(|| vec![])
            })
            .collect::<Vec<StructField>>();

        self.fields.borrow_mut().append(&mut fields);
        self.fields
            .borrow_mut()
            .retain(|field| field.name.as_str() != tag::BASE);

        for subtype in &self.subtypes {
            if let RsEntity::Struct(s) = subtype {
                s.extend_base(types);
            }
        }
    }
}

impl fmt::Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{comment}pub struct {name} {{\n{fields}\n}}\n{subtypes}\n{fields_subtypes}",
            comment = get_formatted_comment(self.comment.as_deref()),
            name = self.name,
            fields = self
                .fields
                .borrow()
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
            subtypes = self
                .subtypes
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
            fields_subtypes = self
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

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub type_name: String,
    pub comment: Option<String>,
    pub subtypes: Vec<RsEntity>,
    pub source: StructFieldSource
}

#[derive(Debug, Clone)]
pub enum StructFieldSource {
    Attribute,
    Element,
    Base,
    NA
}

impl fmt::Display for StructField {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{comment}  pub {name}: {typename},",
            name = self.name,
            typename = self.type_name,
            comment = get_formatted_comment(self.comment.as_deref())
        )
    }
}

#[derive(Debug, Clone)]
pub struct TupleStruct {
    pub name: String,
    pub comment: Option<String>,
    pub type_name: String,
    pub subtypes: Vec<RsEntity>,
}

impl fmt::Display for TupleStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{comment}pub struct {name} (pub {typename});\n{subtypes}",
            comment = get_formatted_comment(self.comment.as_deref()),
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

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub cases: Vec<EnumCase>,
    pub comment: Option<String>,
    pub type_name: String,
    pub subtypes: Vec<RsEntity>,
}

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{comment}\
            #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]\n\
            pub enum {name} \
            {{\n{cases}  \n\n\
            __Unknown__({typename})\n\
            }}\n\n\
            {default}\n\n\
            {subtypes}",
            comment = get_formatted_comment(self.comment.as_deref()),
            name = self.name,
            cases = self
                .cases
                .iter()
                .map(|case| case.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
            typename = self.type_name,
            default = format!("impl Default for {name} {{\n  fn default() -> {name} {{\n    Self::__Unknown__(\"No valid variants\".into())\n  }}\n}}",
                name = self.name
            ),
            subtypes = self
                .subtypes
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
        )
    }
}

#[derive(Debug, Clone)]
pub struct EnumCase {
    pub name: String,
    pub comment: Option<String>,
    pub value: String,
    pub type_name: Option<String>,
}

impl fmt::Display for EnumCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let name = get_type_name(self.name.as_str());
        match &self.type_name {
            Some(tn) => write!(
                f,
                "{comment}  {name}({typename}),",
                name = name,
                typename = tn,
                comment = get_formatted_comment(self.comment.as_deref()),
            ),
            None => write!(
                f,
                "{comment}  {name},",
                name = name,
                comment = get_formatted_comment(self.comment.as_deref())
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub original: String,
    pub comment: Option<String>,
    pub subtypes: Vec<RsEntity>,
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let is_same_name = self.name == self.original;
        writeln!(
            f,
            "{comment}{visibility}type {name} = {original};",
            visibility = if is_same_name { "// " } else { "// pub " }, // TODO: Always commented as an experiment
            comment = get_formatted_comment(self.comment.as_deref()),
            name = self.name,
            original = self.original
        )
    }
}

#[derive(Debug, Clone)]
pub struct Import {
    pub name: String,
    pub location: String,
    pub comment: Option<String>,
}

impl fmt::Display for Import {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "//use {}  {};", self.location, self.name,)
    }
}

#[derive(Debug, Clone)]
pub enum RsEntity {
    Struct(Struct),
    StructField(StructField),
    TupleStruct(TupleStruct),
    Enum(Enum),
    EnumCase(EnumCase),
    Alias(Alias),
    Import(Import),
}

impl fmt::Display for RsEntity {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use RsEntity::*;
        match self {
            Struct(s) => write!(f, "{}", s),
            TupleStruct(tp) => write!(f, "{}", tp),
            Enum(e) => write!(f, "{}", e),
            EnumCase(ec) => write!(f, "{}", ec),
            Alias(al) => write!(f, "{}", al),
            StructField(sf) => write!(f, "{}", sf),
            Import(im) => write!(f, "{}", im),
        }
    }
}

impl RsEntity {
    pub fn name(&self) -> &str {
        use RsEntity::*;
        match self {
            Struct(s) => s.name.as_str(),
            TupleStruct(tp) => tp.name.as_str(),
            Enum(e) => e.name.as_str(),
            EnumCase(ec) => ec.name.as_str(),
            Alias(al) => al.name.as_str(),
            StructField(sf) => sf.name.as_str(),
            Import(im) => im.name.as_str(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        use RsEntity::*;
        match self {
            Struct(s) => s.name = name.to_string(),
            TupleStruct(tp) => tp.name = name.to_string(),
            Enum(e) => e.name = name.to_string(),
            EnumCase(ec) => ec.name = name.to_string(),
            Alias(al) => al.name = name.to_string(),
            StructField(sf) => sf.name = name.to_string(),
            Import(im) => im.name = name.to_string(),
        }
    }

    pub fn set_comment(&mut self, comment: Option<String>) {
        use RsEntity::*;
        match self {
            Struct(s) => s.comment = comment,
            TupleStruct(tp) => tp.comment = comment,
            Enum(e) => e.comment = comment,
            EnumCase(ec) => ec.comment = comment,
            Alias(al) => al.comment = comment,
            StructField(sf) => sf.comment = comment,
            Import(im) => im.comment = comment,
        }
    }
}
