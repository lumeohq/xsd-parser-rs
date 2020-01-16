extern crate inflector;
use inflector::cases::pascalcase::to_pascal_case;
use std::str;
use self::inflector::cases::snakecase::to_snake_case;
use std::borrow::Cow;
use crate::xsd2::node_types::{UseType, Attribute};

fn split_comment_line(s: &str, max_len: usize, indent: usize) -> String {
    let indent_str = " ".repeat(indent);

    let mut splitted = format!("{}//", indent_str);
    let mut current_line_length = indent + 2;
    for word in s.split_whitespace() {
        let len = word.len();
        if current_line_length + len + 1 <= max_len {
            splitted = format!("{} {}", splitted, word);
            current_line_length += 1 + len;
        }
        else {
            splitted = format!("{}\n{}// {}", splitted, indent_str, word);
            current_line_length = indent + 3 + len;
        }
    }
    format!("{}\n", splitted)
}

pub fn get_structure_comment(doc: Option<&str>) -> String {
    doc.
        unwrap_or("").
        lines().
        map(|s| s.trim()).
        filter(|s| s.len() > 2).
        map(|s| split_comment_line(s, 80, 0)).
        fold(String::new(), |x , y| (x+&y))
}

pub fn get_field_comment(doc: Option<&str>) -> String {
    doc.
        unwrap_or("").
        lines().
        map(|s| s.trim()).
        filter(|s| s.len() > 1).
        map(|s| split_comment_line(s, 80, 2)).
        fold(String::new(), |x , y| (x+&y))
}

pub fn match_type(typename: &str, target_namespace: Option<&str>) -> Cow<'static, str>{
    match typename {
        "xs:string"      => Cow::Borrowed("String"),
        "xs:NCName"      => Cow::Borrowed("String"),
        "xs:unsignedInt" => Cow::Borrowed("usize"),
        "xs:int"         => Cow::Borrowed("i64"),
        "xs:float"       => Cow::Borrowed("f64"),
        "xs:boolean"     => Cow::Borrowed("bool"),
        x => Cow::Owned(
            to_pascal_case(
                match target_namespace {
                    Some(ns) => {
                        if x.starts_with(ns) { x[ns.len()+1..].to_string() }
                        else { x.replace(":", "::") }
                    },
                    None => x.replace(":", "::")
                }.as_str()
            )
        )
    }
}

pub fn get_field_name(name: &str) -> String {
    to_snake_case(name)
}

pub fn yaserde_derive() -> String {
    "#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]\n\
        #[yaserde(\n\
          prefix = \"unknown\",\n\
          namespace = \"unknown: unknown\"\n\
        )]\n".to_string()
}

pub fn attribute_type(attr: &Attribute, typename: Cow<str>) -> String {
    match attr.use_type() {
        UseType::Required => typename.to_string(),
        UseType::Optional => format!("Option<{}>", typename),
        UseType::Prohibited => format!("Empty<{}>", typename),
    }
}
