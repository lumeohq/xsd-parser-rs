use core::fmt;

#[derive(Default)]
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

#[derive(Default)]
pub struct Enum {
    pub name: String,
    pub cases: Vec<EnumCase>,
    pub comment: String,
    pub type_name: String
}

impl Enum {
    pub fn to_enum(&self) -> String {
        format!("{body}\n\n{impl}", body=self.enum_body(), impl=self.enum_impl())
    }

    fn enum_body(&self) -> String {
        format!("{comment}{derive}pub enum {name} {{\n{cases}\n\n  __Unknown__({type_name})\n}}",
            comment=self.comment,
            derive="#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]\n",
            name=self.name,
            cases=self.cases.
                iter().
                map(|case| case.case_line()).
                collect::<Vec<String>>().
                join("\n\n"),
            type_name=self.type_name
        )
    }

    fn enum_impl(&self) -> String {
        format!("impl Default for {name} {{\n  fn default() -> {name} {{\n    {name}::__Unknown__(\"No valid variants\".into())\n  }}\n}}",
            name = self.name
        )
    }
}

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,"{}\n", self.to_enum())
    }
}

#[test]
fn enum_default_trait_generation_test() {
    let output = "impl Default for E {\n  fn default() -> E {\n    E::__Unknown__(\"No valid variants\".into())\n  }\n}";

    // No cases except for __Unknown__.
    let mut e = Enum{
        type_name: "String".to_string(),
        name: "E".to_string(),
        ..Default::default()
    };
    assert_eq!(e.enum_impl(), output);

    // Some cases.
    e.cases = vec![EnumCase {
        name: "EC".to_string(),
        ..Default::default()
    }];
    assert_eq!(e.enum_impl(), output);
}
