
use std::fmt;
use crate::xsd::utils::*;
use crate::xsd::elements::*;

struct Attribute {
    name: String,
    type_: String,
    documentation: Option<String>,
}

impl Attribute {
    pub fn new(node: & roxmltree::Node) -> Self {
        let get_type = ||  match_type(node.attribute("type").unwrap_or("UNSUPPORTED_TYPE_ATTRIBUTE")).to_string();

        let type_= match node.attribute("use") {
            Some("required") => get_type(),
            _ => format!("Option<{}>", get_type())
        }.replace(":","::");

        Attribute {
            name: lowercase_first_letter(node.attribute("name").unwrap_or("_DEFAULT_NAME_FOR_ATTRIBUTE")),
            type_,
            documentation: get_documentation(node).map(|v| v.to_string()),
        }
    }
}

impl GenerateCode for Attribute {
    fn generate_code(&self) -> String {
        format!("{}  {}: {},", get_struct_comment(Some(
            self.documentation.as_ref().unwrap_or(&"".to_string()))),
                self.name,
                self.type_
        )
    }
}

fn get_attrs<'a>(node: &'a roxmltree::Node) -> Vec<Attribute> {
    node.children().
        filter(|e| e.is_element() && e.tag_name().name() == "attribute").
        map(|e| Attribute::new(&e)).collect()
}

struct Element {
    name: String,
    type_: String,
    documentation: Option<String>
}

impl Element {
    pub fn new(node: & roxmltree::Node) -> Self {
        let name = node.attribute("name").expect("Name required for element").to_string();
        let t = node.attribute("type").unwrap_or("NESTED_TYPE_UNSUPPORTED");
        let is_vec = match node.attribute("maxOccurs") {
            Some("unbounded") => true,
            Some(s) => match s.parse::<u32>() {
                Ok(v) => v > 1,
                _ => false
            }
            _ => false
        };

        let mut type_ = match_type(t).replace(":", "::");

        if is_vec {
            type_ = format!("Vec<{}>", type_);
        }

        Element {
            name,
            type_,
            documentation: get_documentation(node).map(|t| t.to_string())
        }
    }
}

impl GenerateCode for Element {
    fn generate_code(&self) -> String {
        format!("  {}: {},  //  {}",
                self.name,
                self.type_,
                get_field_comment(&self.documentation),
        )
    }
}

fn get_sequence<'a> (node: &'a roxmltree::Node) -> Vec<Element> {
    match find_child(node, "sequence") {
        Some(node) => node.
            children().
            filter(|n| n.is_element() && n.tag_name().name() == "element").
            map(|e| Element::new(&e)).
            collect(),
        _ => vec!()
    }
}

pub struct ComplexType<'a> {
    name: String,
    documentation: Option<&'a str>,
    attrs: Vec<Attribute>,
    sequence: Vec<Element>
}

impl<'a> ComplexType<'a> {
    pub fn new(node: &'a roxmltree::Node) -> Self {
        let name = uppercase_first_letter(node.attribute("name").unwrap());
        ComplexType{
            name,
            documentation: get_documentation(node),
            attrs: get_attrs(node),
            sequence: get_sequence(node)
        }
    }
}

impl<'a> fmt::Debug for ComplexType <'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.generate_code() )
    }
}

impl GenerateCode for ComplexType<'_> {
    fn generate_code(&self) -> String {
        let mut fields: Vec<String> = self.attrs.iter().
            map(|a| a.generate_code()).
            collect();
        //let mut fields2: Vec<String> = self.sequence.iter().map(|e| e.generate_code()).collect();
        fields.extend::<Vec<String>>(self.sequence.iter().map(|e| e.generate_code()).collect());

        format!("{}pub struct {} {{\n{}\n}} \n\n",
                get_struct_comment(self.documentation),
                self.name,
                fields.join("\n")
        )
    }
}
