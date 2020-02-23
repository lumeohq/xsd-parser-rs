use std::cell::RefCell;
use std::collections::HashMap;

use crate::parser::constants::tag;
use roxmltree::Namespace;

#[derive(Debug, Clone)]
pub struct File<'input> {
    pub name: String,
    pub namespace: Option<String>,
    pub types: Vec<RsEntity>,
    pub target_ns: Option<Namespace<'input>>,
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

#[derive(Debug, Clone, Default)]
pub struct StructField {
    pub name: String,
    pub type_name: String,
    pub comment: Option<String>,
    pub subtypes: Vec<RsEntity>,
    pub source: StructFieldSource,
    pub type_modifiers: Vec<TypeModifier>,
}

#[derive(Debug, Clone)]
pub enum StructFieldSource {
    Attribute,
    Element,
    Base,
    NA,
}

impl Default for StructFieldSource {
    fn default() -> Self {
        StructFieldSource::NA
    }
}


#[derive(Debug, Clone, Default)]
pub struct TupleStruct {
    pub name: String,
    pub comment: Option<String>,
    pub type_name: String,
    pub subtypes: Vec<RsEntity>,
    pub type_modifiers: Vec<TypeModifier>,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub cases: Vec<EnumCase>,
    pub comment: Option<String>,
    pub type_name: String,
    pub subtypes: Vec<RsEntity>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeModifier {
    None,
    Array,
    Option,
    Empty,
}

#[derive(Debug, Clone)]
pub struct EnumCase {
    pub name: String,
    pub comment: Option<String>,
    pub value: String,
    pub type_name: Option<String>,
    pub type_modifiers: Vec<TypeModifier>,
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub original: String,
    pub comment: Option<String>,
    pub subtypes: Vec<RsEntity>,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub name: String,
    pub location: String,
    pub comment: Option<String>,
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
