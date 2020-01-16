use core::fmt;

pub struct EnumCase {
    pub name: String,
    pub comment: String,
    pub value: String,
    pub type_name: Option<String>
}

impl EnumCase {
    pub fn case_line(&self) -> String {
        let line = match &self.type_name {
            Some(type_name ) => format!("  {name}({type_name}),",
                name=self.name,
                type_name=type_name,
            ),
            None => format!("  {name},",
                name=self.name,
            )
        };
        if self.comment.is_empty() {
            line
        }
        else {
            format!("{comment}{line}", comment=self.comment, line=line)
        }
    }
}

pub struct Enum {
    pub name: String,
    pub cases: Vec<EnumCase>,
    pub comment: String,
    pub type_name: String
}

impl Enum {
    pub fn to_enum(&self) -> String {
        format!("{comment}\npub enum {name} {{\n{cases}  \n__Unknown__({type_name})\n}}",
            comment=self.comment,
            name=self.name,
            cases=self.cases.
                iter().
                map(|case| case.case_line()).
                collect::<Vec<String>>().
                join("\n"),
            type_name=self.type_name
        )
    }
}

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,"{}\n", self.to_enum())
    }
}
