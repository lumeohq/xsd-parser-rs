use core::fmt;

pub struct StructField {
    pub name: String,
    pub typename: String,
    pub comment: String,
    pub macros: String
}

impl fmt::Display for StructField {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "{macros}  pub {name}: {typename},  {comment}",
               macros=self.macros,
               name=self.name,
               typename=self.typename,
               comment=self.comment
        )
    }
}

pub struct Struct {
    pub name: String,
    pub comment: String,
    pub fields: Vec<StructField>,
    pub macros: String
}

impl fmt::Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "{comment}{macros}pub struct {name} {{\n {fields}\n}}\n",
               comment=self.comment,
               macros=self.macros,
               name=self.name,
               fields=self.fields.iter().map(|f| f.to_string()).collect::<Vec<String>>().join("\n")
        )
    }
}

pub struct TupleStruct {
    pub name: String,
    pub comment: String,
    pub typename: String,
    pub macros: String
}

impl fmt::Display for TupleStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "{comment}{macros}pub struct {name} ({typename});\n",
               comment=self.comment,
               macros=self.macros,
               name=self.name,
               typename=self.typename
        )
    }
}

pub struct EnumCase {
    pub name: String,
    pub comment: String,
    pub value: String
}

impl EnumCase {
    pub fn case_line(&self) -> String {
        format!("  {name},  {comment}",
                name=self.name,
                comment=self.comment
        )
    }

    pub fn match_line(&self) -> String {
        format!("      \"{value}\" => Self::{name},", value=self.value, name=self.name)
    }
}

pub struct Enum {
    pub name: String,
    pub cases: Vec<EnumCase>,
    pub comment: String,
    pub typename: String
}

impl Enum {
    pub fn to_enum(&self) -> String {
        format!("{comment}\npub enum {name} {{\n{cases}  \n__Unknown__({typename})\n}}",
            comment=self.comment,
            name=self.name,
            cases=self.cases.
                iter().
                map(|case| case.case_line()).
                collect::<Vec<String>>().
                join("\n"),
            typename=self.typename
        )
    }

    pub fn to_impl(&self) -> String {
        format!(r#"
impl {name} {{
  pub fn new(s: &str) -> Self {{
    match s {{
{lines}
      value => Self::__Unknown__(value.to_string()),
    }}
  }}
}}"#,
        name=self.name,
        lines=self.cases.
            iter().
            map(|case| case.match_line()).
            collect::<Vec<String>>().
            join("\n")
        )
    }
}

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,"{}\n{}\n", self.to_enum(), self.to_impl())
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
