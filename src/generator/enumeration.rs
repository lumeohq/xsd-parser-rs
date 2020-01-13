use core::fmt;

pub struct EnumCase {
    pub name: String,
    pub comment: String,
    pub value: String,
    pub typename: Option<String>
}

impl EnumCase {
    pub fn case_line(&self) -> String {
        match &self.typename {
            Some(ty) => format!("  {name}({typename}),  {comment}",
                name=self.name,
                typename=ty,
                comment=self.comment,
            ),
            None => format!("  {name},  {comment}",
                name=self.name,
                comment=self.comment
            )
        }

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
}

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,"{}\n", self.to_enum())
    }
}
