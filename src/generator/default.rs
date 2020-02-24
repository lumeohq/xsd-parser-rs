use crate::generator::utils::{match_built_in_type, sanitize, split_comment_line};
use crate::parser::types::TypeModifier;
use inflector::cases::pascalcase::to_pascal_case;
use inflector::cases::snakecase::to_snake_case;
use roxmltree::Namespace;
use std::borrow::Cow;

pub fn default_format_comment(doc: Option<&str>, max_len: usize, indent: usize) -> String {
    doc.unwrap_or("")
        .lines()
        .map(|s| s.trim())
        .filter(|s| s.len() > 1)
        .map(|s| split_comment_line(s, max_len, indent))
        .fold(String::new(), |x, y| (x + &y))
}

pub fn default_format_name(name: &str) -> String {
    sanitize(to_snake_case(name))
}

pub fn default_format_type(type_name: &str, target_ns: &Option<Namespace>) -> Cow<'static, str> {
    fn replace(type_name: &str) -> String {
        match type_name.find(':') {
            Some(index) => format!(
                "{}::{}",
                &type_name[0..index],
                to_pascal_case(&type_name[index..])
            ),
            None => to_pascal_case(type_name),
        }
    }

    {
        let built_in_type = match_built_in_type(type_name);
        if !built_in_type.is_empty() {
            return built_in_type.into();
        }
    }

    sanitize(match target_ns.as_ref().and_then(|ns| ns.name()) {
        Some(name) => {
            if type_name.starts_with(name) {
                to_pascal_case(&type_name[name.len() + 1..])
            } else {
                replace(type_name)
            }
        }
        None => replace(type_name),
    })
    .into()
}

pub fn default_modify_type(type_name: &str, modifiers: &[TypeModifier]) -> Cow<'static, str> {
    if modifiers.contains(&TypeModifier::Empty) {
        return "()".into();
    }
    if modifiers.contains(&TypeModifier::Recursive) {
        return format!("Vec<{}>", type_name).into();
    }

    let mut result = type_name.to_string();
    for modifier in modifiers {
        match modifier {
            TypeModifier::Array => result = format!("Vec<{}>", result),
            TypeModifier::Option => result = format!("Option<{}>", result),
            _ => (),
        }
    }
    result.into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_format_comment() {
        let doc = Some(
            r#"
        Line of documentation!
        New line of documentation with len>30 symbols!


        And new line after empty lines"#,
        );

        let expected = r#"  // Line of documentation!
  // New line of documentation
  // with len>30 symbols!
  // And new line after empty
  // lines
"#;
        assert_eq!(default_format_comment(doc, 30, 2), expected);
    }

    #[test]
    fn test_default_format_name() {
        assert_eq!(default_format_name("Struct").as_str(), "_struct");
        assert_eq!(default_format_name("StructName").as_str(), "struct_name");
        assert_eq!(default_format_name("0fst").as_str(), "_0fst");
    }

    #[test]
    fn test_default_format_type_without_target_ns() {
        assert_eq!(default_format_type("Type", &None), "Type");
        assert_eq!(default_format_type("TyName", &None), "TyName");
        assert_eq!(default_format_type("Ty_Name", &None), "TyName");
        assert_eq!(default_format_type("Ty-Name", &None), "TyName");
        assert_eq!(default_format_type("tt:TyName", &None), "tt::TyName");
        assert_eq!(default_format_type("0type_name", &None), "_0TypeName");
        assert_eq!(default_format_type("Enum", &None), "Enum");

        assert_eq!(default_format_type("xs:integer", &None), "i64");
        assert_eq!(default_format_type("xs:nonNegativeInteger", &None), "u64");
    }

    #[test]
    fn test_default_format_type_with_target_ns() {
        let ns = Some(
            roxmltree::Document::parse(
                r#"<schema
            xmlns:tt="http://www.onvif.org/ver10/schema"
            targetNamespace="http://www.onvif.org/ver10/schema"/>"#,
            )
            .unwrap()
            .root_element()
            .namespaces()[0]
                .clone(),
        );
        assert_eq!(default_format_type("tt:Type", &ns), "Type");
        assert_eq!(default_format_type("tt:TyName", &ns), "TyName");
        assert_eq!(default_format_type("tt:Ty_Name", &ns), "TyName");
        assert_eq!(default_format_type("tt:Ty-Name", &ns), "TyName");
        assert_eq!(default_format_type("tt:0type_name", &ns), "_0TypeName");
        assert_eq!(default_format_type("tt:0_type-Name", &ns), "_0TypeName");
        assert_eq!(default_format_type("tt:IANA_IfTypes ", &ns), "IanaIfTypes");
        assert_eq!(default_format_type("tt:Enum", &ns), "Enum");
        assert_eq!(default_format_type("xs:TyName", &ns), "xs::TyName");
    }

    #[test]
    fn test_default_modify_type() {
        use TypeModifier::*;
        assert_eq!(default_modify_type("Type", &[Recursive]), "Vec<Type>");
        assert_eq!(default_modify_type("Type", &[None]), "Type");
        assert_eq!(default_modify_type("Type", &[Option]), "Option<Type>");
        assert_eq!(default_modify_type("Type", &[Array]), "Vec<Type>");
        assert_eq!(default_modify_type("Type", &[Empty]), "()");

        assert_eq!(
            default_modify_type("Type", &[Recursive, Option]),
            "Vec<Type>"
        );
        assert_eq!(
            default_modify_type("Type", &[Recursive, Array, Option]),
            "Vec<Type>"
        );
        assert_eq!(
            default_modify_type("Type", &[Recursive, Array, Empty]),
            "()"
        );
    }
}
