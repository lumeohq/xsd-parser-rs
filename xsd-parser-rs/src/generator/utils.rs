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

pub fn match_built_in_type(type_name: &str) -> &'static str {
    match type_name {
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

        // TODO: might use types from chrono crate instead, but with a wrap that implements Yaserde
        // (de)serialization. For that we need to use the "flatten" from yaserde, as it will be
        // updated on the crates.io.
        "xs:date" => "String",
        "xs:time" => "String",
        "xs:dateTime" => "String",
        "xs:dateTimeStamp" => "String",

        "xs:duration" => "xs::Duration",

        // TODO: would be nice to have types with both numeric representation of value and proper
        // (de)serialization. For that we might use the "flatten" from yaserde, as it will be
        // updated on the crates.io.
        "xs:gDay" => "String",
        "xs:gMonth" => "String",
        "xs:gMonthDay" => "String",
        "xs:gYear" => "String",
        "xs:gYearMonth" => "String",

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
