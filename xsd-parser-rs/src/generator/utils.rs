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

pub fn match_built_in_type(type_name: &str) -> Option<&'static str> {
    let res = match type_name {
        "xs:hexBinary" => "String",
        "xs:base64Binary" => "String",

        "xs:boolean" => "bool",

        "xs:integer" => "xs::Integer",
        "xs:nonNegativeInteger" => "xs::Integer",
        "xs:positiveInteger" => "xs::Integer",
        "xs:nonPositiveInteger" => "xs::Integer",
        "xs:negativeInteger" => "xs::Integer",

        "xs:long" => "i64",
        "xs:int" => "i32",
        "xs:short" => "i16",
        "xs:byte" => "i8",

        "xs:unsignedLong" => "u64",
        "xs:unsignedInt" => "u32",
        "xs:unsignedShort" => "u16",
        "xs:unsignedByte" => "u8",

        "xs:decimal" => "xs::Decimal",

        "xs:double" => "f64",
        "xs:float" => "f64",

        "xs:date" => "xs::Date",
        "xs:time" => "xs::Time",
        "xs:dateTime" => "xs::DateTime",
        "xs:dateTimeStamp" => "xs::DateTimeStamp",

        "xs:duration" => "xs::Duration",

        "xs:gDay" => "xs::GDay",
        "xs:gMonth" => "xs::GMonth",
        "xs:gMonthDay" => "xs::GMonthDay",
        "xs:gYear" => "xs::GYear",
        "xs:gYearMonth" => "xs::GYearMonth",

        "xs:string" => "String",
        "xs:normalizedString" => "String",
        "xs:token" => "String",
        "xs:language" => "String",
        "xs:Name" => "String",
        "xs:NCName" => "String",
        "xs:ENTITY" => "String",
        "xs:ID" => "String",
        "xs:IDREF" => "String",
        "xs:NMTOKEN" => "String",
        "xs:anyURI" => "String",
        "xs:QName" => "String",

        "xs:NOTATION" => "String",

        "xs:anySimpleType" => "String",

        // Built-in list types:
        "xs:ENTITIES" => "Vec<String>",
        "xs:IDREFS" => "Vec<String>",
        "xs:NMTOKENS" => "Vec<String>",
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
    use crate::generator::utils::{filter_type_name, split_name};

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
}
