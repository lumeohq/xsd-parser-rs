extern crate inflector;
use self::inflector::cases::snakecase::to_snake_case;
use crate::generator2::types::StructField;
use inflector::cases::pascalcase::to_pascal_case;
use roxmltree::{Node, Namespace};
use std::borrow::Cow;
use std::str;

pub fn split_comment_line(s: &str, max_len: usize, indent: usize) -> String {
    let indent_str = " ".repeat(indent);

    let mut splitted = format!("{}//", indent_str);
    let mut current_line_length = indent + 2;
    for word in s.split_whitespace() {
        let len = word.len();
        if current_line_length + len + 1 <= max_len || current_line_length == indent + 2 {
            splitted = format!("{} {}", splitted, word);
            current_line_length += 1 + len;
        } else {
            splitted = format!("{}\n{}// {}", splitted, indent_str, word);
            current_line_length = indent + 3 + len;
        }
    }
    format!("{}\n", splitted)
}

pub fn get_structure_comment(doc: Option<&str>) -> String {
    doc.unwrap_or("")
        .lines()
        .map(|s| s.trim())
        .filter(|s| s.len() > 2)
        .map(|s| split_comment_line(s, 80, 0))
        .fold(String::new(), |x, y| (x + &y))
}

pub fn get_field_comment(doc: Option<&str>) -> String {
    doc.unwrap_or("")
        .lines()
        .map(|s| s.trim())
        .filter(|s| s.len() > 1)
        .map(|s| format!("// {}  ", s))
        .fold(String::new(), |x, y| (x + &y))
}

pub fn match_type(typename: &str, target_namespace: Option<&str>) -> Cow<'static, str> {
    match typename {
        "xs:string" => Cow::Borrowed("String"),
        "xs:NCName" => Cow::Borrowed("String"),
        "xs:unsignedInt" => Cow::Borrowed("usize"),
        "xs:int" => Cow::Borrowed("i64"),
        "xs:float" => Cow::Borrowed("f64"),
        "xs:boolean" => Cow::Borrowed("bool"),
        x => Cow::Owned(to_pascal_case(
            match target_namespace {
                Some(ns) => {
                    if x.starts_with(ns) {
                        x[ns.len() + 1..].to_string()
                    } else {
                        x.replace(":", "::")
                    }
                }
                None => x.replace(":", "::"),
            }
            .as_str(),
        )),
    }
}

pub fn get_field_name(name: &str) -> String {
    to_snake_case(name)
}

pub fn any_attribute_field() -> StructField {
    StructField {
        name: "any_attribute".to_string(),
        type_name: "AnyAttribute".to_string(),
        comment: None,
        macros: "//TODO: yaserde macros for any attribute\n".to_string(),
    }
}

fn any_element_field() -> StructField {
    StructField {
        name: "any_element".to_string(),
        type_name: "AnyElement".to_string(),
        macros: "//TODO: yaserde macros for any element\n".to_string(),
        comment: None,
    }
}

pub fn target_namespace<'a, 'input>(node: &Node<'a, 'input>) -> Option<&'a Namespace<'input>> {
    match node.attribute("targetNamespace") {
        Some(tn) => node
            .namespaces()
            .iter()
            .find(|a| a.uri() == tn),
        None => None,
    }
}

pub fn find_child<'a, 'input>(node: &Node<'a, 'input>, tag_name: &str) -> Option<Node<'a, 'input>> {
    node.children()
        .find(|e| e.is_element() && e.tag_name().name() == tag_name)
}

pub fn get_documentation<'a>(node: &Node<'a, '_>) -> Option<String> {
    find_child(node, "annotation")
        .and_then(|node| find_child(&node, "documentation"))
        .and_then(|node| node.text().map(|s| s.to_string()))
}

pub fn tuple_struct_macros() -> String {
    "//TODO: Tuple Struct macros\n".to_string()
}

pub fn struct_field_macros() -> String {
    "//TODO: struct_field_macros\n".to_string()
}

pub fn struct_macros() -> String {
    "//TODO: struct macros\n".to_string()
}

pub fn get_parent_name(node: Option<Node>) -> String {
    match node {
        Some(n) => match n.attribute("name") {
            Some(s) => s.to_string(),
            None => get_parent_name(n.parent()).to_string() + "_" + n.tag_name().name(),
        },
        None => "UnsupportedName".to_string(),
    }
}


