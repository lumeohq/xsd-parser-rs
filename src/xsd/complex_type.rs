use std::fmt;
use crate::xsd::utils::*;
use crate::xsd::traits::*;
use crate::xsd::element::Element;
use crate::xsd::attribute::Attribute;


pub struct ComplexType<'a> {
    name: String,
    documentation: Option<&'a str>,
    attrs: Vec<Attribute>,
    sequence: Vec<Element>,
    has_any_elements: bool,
    has_any_attributes: bool,
}

impl<'a> ComplexType<'a> {
    pub fn new(node: &'a roxmltree::Node) -> Self {
        let name = uppercase_first_letter(node.attribute("name").expect("Empty name ComplexType"));
        ComplexType{
            name,
            documentation: get_documentation(node),
            attrs: get_attrs(node),
            sequence: get_sequence(node),
            has_any_elements: has_any_elements(node),
            has_any_attributes: has_any_attributes(node)
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
        let mut fields: Vec<String> = self.sequence.
            iter().
            map(|e| e.generate_code()).
            collect();

        fields.extend::<Vec<String>>(self.attrs.
            iter().
            map(|a| a.generate_code()).
            collect()
        );
        if self.has_any_elements {
            fields.push("  any_elements: Vec<AnyElement>,".to_string() );
        }

        if self.has_any_attributes {
            fields.push("  any_attributes: HashMap<String, String>,".to_string() );
        }

        format!("{}pub struct {} {{\n{}\n}} \n\n",
                get_struct_comment(self.documentation),
                self.name,
                fields.join("\n")
        )
    }
}

fn get_attrs<'a>(node: &'a roxmltree::Node) -> Vec<Attribute> {
    node.children().
        filter(|e| e.is_element() && e.tag_name().name() == "attribute").
        map(|e| Attribute::new(&e)).collect()
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

fn has_any_elements(node: &roxmltree::Node) -> bool {
    match find_child(node, "sequence") {
        Some(node) => find_child(&node, "any").is_some(),
        _ => false
    }
}

fn has_any_attributes(node: &roxmltree::Node) -> bool {
    find_child(node, "anyAttribute").is_some()
}
