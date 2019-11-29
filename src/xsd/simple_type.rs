
use std::fmt;
use crate::xsd::utils::*;
use crate::xsd::elements::*;

pub struct SimpleType <'a> {
    name: String,
    documentation: Option<&'a str>,
    field_type: String
}

impl<'a> SimpleType<'a> {
    pub fn new(node: &'a roxmltree::Node) -> Self {
        let name = uppercase_first_letter(node.attribute("name").unwrap());
        let field_type = match find_child(node, "restriction") {
            Some(node) => node.attribute("base").unwrap().to_string(),
            None => match find_child(node, "list") {
                Some(node) => format!("Vec<{}>", match_type(node.attribute("itemType").unwrap())),
                None => "UNKNOWN TYPE".to_string()
            }
        };
        SimpleType{
            name,
            documentation: get_documentation(node),
            field_type: match_type(field_type.as_str()).replace(":", "::")
        }
    }
}

impl<'a> fmt::Debug for SimpleType <'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.generate_code())
    }
}

impl GenerateCode for SimpleType<'_> {
    fn generate_code(&self) -> String {
        format!("{} pub struct {} ({}); \n\n",
                get_struct_comment(self.documentation),
                self.name,
                self.field_type
        )
    }
}
