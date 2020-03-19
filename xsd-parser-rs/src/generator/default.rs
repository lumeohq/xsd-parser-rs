use crate::generator::utils::{match_built_in_type, sanitize, split_comment_line, split_name, filter_type_name};
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
    sanitize(to_snake_case(name.split(':').last().unwrap()))
}

pub fn default_format_type(type_name: &str, target_ns: &Option<Namespace>) -> Cow<'static, str> {
    if let Some(t) = match_built_in_type(type_name) {
        return t.into();
    }

    let (option_ns, ty) = split_name(type_name);
    let option_tns = target_ns.as_ref().and_then(|ns| ns.name());

    let pascalized_ty = to_pascal_case(filter_type_name(ty).as_str());

    let qname = |ns| format!("{}::{}", ns, pascalized_ty);

    let res = match (option_ns, option_tns) {
        (Some(ns), Some(tns)) => {
            if ns == tns {
                pascalized_ty
            } else {
                qname(ns)
            }
        }
        (Some(ns), None) => qname(ns),
        _ => pascalized_ty
    };

    sanitize(res).into()
}

pub fn default_format_enum_case_name(
    type_name: &str,
    target_ns: &Option<Namespace>,
) -> Cow<'static, str> {
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

        // Name and type from 'ref' attribute
        // <xs:element ref="xop:Include"/>
        assert_eq!(default_format_name("xop:Include").as_str(), "include");
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

        assert_eq!(default_format_type("xs:integer", &None), "xs::Integer");
        assert_eq!(
            default_format_type("xs:nonNegativeInteger", &None),
            "xs::Integer"
        );
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
        assert_eq!(default_format_type("ttEnum", &ns), "TtEnum");
        assert_eq!(default_format_type("xs:TyName", &ns), "xs::TyName");

        assert_eq!(default_format_type("http://www.w3.org/2005/08/addressing/reply", &ns),
                   "http::Wwww3Org200508Addressingreply");
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
