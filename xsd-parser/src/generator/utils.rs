use roxmltree::Namespace;

pub fn split_comment_line(s: &str, max_len: usize, indent: usize) -> String {
    let indent_str = " ".repeat(indent);

    let mut splitted = format!("{}//", indent_str);
    let mut current_line_length = indent + 2;
    for word in s.split_whitespace() {
        let len = word.len();
        if current_line_length + len < max_len {
            splitted = format!("{} {}", splitted, word);
            current_line_length += 1 + len;
        } else {
            splitted = format!("{}\n{}// {}", splitted, indent_str, word);
            current_line_length = indent + 3 + len;
        }
    }
    format!("{}\n", splitted)
}

pub fn match_built_in_type(type_name: &str, xsd_ns: &Option<Namespace>) -> Option<&'static str> {
    let (prefix, name) = split_name(type_name);
    let xsd_prefix = xsd_ns.as_ref().and_then(|ns| ns.name());
    if xsd_prefix != prefix {
        return None;
    }
    let res = match name {
        "hexBinary" => "String",
        "base64Binary" => "String",

        "boolean" => "bool",

        "integer" => "xs::Integer",
        "nonNegativeInteger" => "xs::Integer",
        "positiveInteger" => "xs::Integer",
        "nonPositiveInteger" => "xs::Integer",
        "negativeInteger" => "xs::Integer",

        "long" => "i64",
        "int" => "i32",
        "short" => "i16",
        "byte" => "i8",

        "unsignedLong" => "u64",
        "unsignedInt" => "u32",
        "unsignedShort" => "u16",
        "unsignedByte" => "u8",

        "decimal" => "xs::Decimal",

        "double" => "f64",
        "float" => "f64",

        "date" => "xs::Date",
        "time" => "xs::Time",
        "dateTime" => "xs::DateTime",
        "dateTimeStamp" => "xs::DateTimeStamp",

        "duration" => "xs::Duration",

        "gDay" => "xs::GDay",
        "gMonth" => "xs::GMonth",
        "gMonthDay" => "xs::GMonthDay",
        "gYear" => "xs::GYear",
        "gYearMonth" => "xs::GYearMonth",

        "string" => "String",
        "normalizedString" => "String",
        "token" => "String",
        "language" => "String",
        "Name" => "String",
        "NCName" => "String",
        "ENTITY" => "String",
        "ID" => "String",
        "IDREF" => "String",
        "NMTOKEN" => "String",
        "anyURI" => "String",
        "QName" => "String",

        "NOTATION" => "String",

        "anySimpleType" => "String",

        // Built-in list types:
        "ENTITIES" => "Vec<String>",
        "IDREFS" => "Vec<String>",
        "NMTOKENS" => "Vec<String>",
        _ => "",
    };

    if res.is_empty() {
        None
    } else {
        Some(res)
    }
}

pub fn sanitize(s: String) -> String {
    if s.is_empty() {
        s
    } else if s.chars().next().unwrap().is_numeric() || RS_KEYWORDS.contains(&s.as_str()) {
        format!("_{}", s)
    } else {
        s
    }
}

pub fn filter_type_name(name: &str) -> String {
    fn is_valid_symbol(c: char) -> bool {
        (c.is_alphanumeric() || c == '_') && c.is_ascii() && !c.is_whitespace()
    }

    name.chars().filter(|c| is_valid_symbol(*c)).collect()
}

pub fn split_name(name: &str) -> (Option<&str>, &str) {
    match name.find(':') {
        Some(index) => (Some(&name[0..index]), &name[index + 1..]),
        None => (None, name),
    }
}

const RS_KEYWORDS: &[&str] = &[
    "abstract",
    "alignof",
    "as",
    "become",
    "box",
    "break",
    "const",
    "continue",
    "crate",
    "do",
    "else",
    "enum",
    "extern crate",
    "extern",
    "false",
    "final",
    "fn",
    "for",
    "if let",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "macro",
    "match",
    "mod",
    "move",
    "mut",
    "offsetof",
    "override",
    "priv",
    "proc",
    "pub",
    "pure",
    "ref",
    "return",
    "Self",
    "self",
    "sizeof",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "typeof",
    "unsafe",
    "unsized",
    "use",
    "virtual",
    "where",
    "while",
    "yield",
];

#[cfg(test)]
mod test {
    use crate::generator::utils::{filter_type_name, match_built_in_type, split_name};

    #[test]
    fn test_filter_type_name() {
        assert_eq!(filter_type_name("SomeType"), "SomeType");
        assert_eq!(filter_type_name("Some-Type"), "SomeType");
        assert_eq!(filter_type_name("Some Type"), "SomeType");
        assert_eq!(filter_type_name("Some@Type"), "SomeType");
        assert_eq!(filter_type_name("Some❤Type"), "SomeType");
        assert_eq!(filter_type_name("Some_Type"), "Some_Type");

        assert_eq!(
            filter_type_name("http://www.w3.org/2005/08/addressing/reply"),
            "httpwwww3org200508addressingreply"
        );
        assert_eq!(
            filter_type_name("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            "httpswwwyoutubecomwatchvdQw4w9WgXcQ"
        );
    }

    #[test]
    fn test_split_name() {
        assert_eq!(split_name("xs:Type"), (Some("xs"), "Type"));
        assert_eq!(split_name("xsType"), (None, "xsType"));
    }

    #[test]
    fn test_match_built_in_types() {
        let xsd_ns = Some(
            roxmltree::Document::parse(
                r#"<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"/>"#,
            )
            .unwrap()
            .root_element()
            .namespaces()[0]
                .clone(),
        );

        let match_type = |name| match_built_in_type(name, &xsd_ns);

        assert_eq!(match_type("xsd:string"), Some("String"));
        assert!(match_type("xs:string").is_none());
    }
}
