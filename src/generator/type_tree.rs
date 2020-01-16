use core::fmt;
use crate::generator::struct_field::StructField;
use crate::generator::enumeration::Enum;

pub struct Struct {
    pub name: String,
    pub comment: String,
    pub fields: Vec<StructField>,
    pub macros: String
}

impl fmt::Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "{comment}{macros}pub struct {name} {{\n{fields}\n}}\n",
               comment=self.comment,
               macros=self.macros,
               name=self.name,
               fields=self.fields.iter().map(|f| f.to_string()).collect::<Vec<String>>().join("\n\n")
        )
    }
}

pub struct TupleStruct {
    pub name: String,
    pub comment: String,
    pub type_name: String,
    pub macros: String
}

impl fmt::Display for TupleStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "{comment}{macros}pub struct {name} ({type_name});\n",
               comment=self.comment,
               macros=self.macros,
               name=self.name,
               type_name=self.type_name
        )
    }
}

pub enum Types {
    Struct(Struct),
    TupleStruct(TupleStruct),
    Enum(Enum)
}

impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Types::Struct(s) => write!(f, "{}", s),
            Types::TupleStruct(tp) => write!(f, "{}", tp),
            Types::Enum(e) => write!(f, "{}", e),
        }
    }
}
