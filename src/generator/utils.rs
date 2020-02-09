extern crate inflector;
use inflector::cases::pascalcase::to_pascal_case;
use std::str;
use self::inflector::cases::snakecase::to_snake_case;
use std::borrow::Cow;
use crate::xsd2::node_types::{UseType, Attribute};

pub fn split_comment_line(s: &str, max_len: usize, indent: usize) -> String {
    let indent_str = " ".repeat(indent);

    let mut splitted = format!("{}//", indent_str);
    let mut current_line_length = indent + 2;
    for word in s.split_whitespace() {
        let len = word.len();
        if current_line_length + len + 1 <= max_len || current_line_length == indent + 2 {
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

pub fn match_type(type_name: &str, target_namespace: Option<&roxmltree::Namespace>) -> Cow<'static, str>{
    match type_name {
        "xs:hexBinary"    => "String".into(),
        "xs:base64Binary" => "String".into(),

        "xs:boolean"      => "bool".into(),

        // TODO: should use something like num_bigint::Bigint instead, but with a wrap that
        // implements Yaserde (de)serialization. For that we need to use the "flatten" from yaserde,
        // as it will be updated on the crates.io.
        "xs:integer"            => "i64".into(),
        "xs:nonNegativeInteger" => "u64".into(),
        "xs:positiveInteger"    => "u64".into(),
        "xs:nonPositiveInteger" => "i64".into(),
        "xs:negativeInteger"    => "i64".into(),

        "xs:long"  => "i64".into(),
        "xs:int"   => "i32".into(),
        "xs:short" => "i16".into(),
        "xs:byte"  => "i8".into(),

        "xs:unsignedLong"  => "u64".into(),
        "xs:unsignedInt"   => "u32".into(),
        "xs:unsignedShort" => "u16".into(),
        "xs:unsignedByte"  => "u8".into(),

        // TODO: should use something like bigdecimal::BigDecimal instead, but with a wrap that
        // implements Yaserde (de)serialization. For that we need to use the "flatten" from yaserde,
        // as it will be updated on the crates.io.
        "xs:decimal" => "f64".into(),

        "xs:double" => "f64".into(),
        "xs:float"  => "f64".into(),

        // TODO: might use types from chrono crate instead, but with a wrap that implements Yaserde
        // (de)serialization. For that we need to use the "flatten" from yaserde, as it will be
        // updated on the crates.io.
        "xs:date"          => "String".into(),
        "xs:time"          => "String".into(),
        "xs:dateTime"      => "String".into(),
        "xs:dateTimeStamp" => "String".into(),

        "xs:duration" => "tt:Duration".into(),

        // TODO: would be nice to have types with both numeric representation of value and proper
        // (de)serialization. For that we might use the "flatten" from yaserde, as it will be
        // updated on the crates.io.
        "xs:gDay"       => "String".into(),
        "xs:gMonth"     => "String".into(),
        "xs:gMonthDay"  => "String".into(),
        "xs:gYear"      => "String".into(),
        "xs:gYearMonth" => "String".into(),

        "xs:string"           => "String".into(),
        "xs:normalizedString" => "String".into(),
        "xs:token"            => "String".into(),
        "xs:language"         => "String".into(),
        "xs:Name"             => "String".into(),
        "xs:NCName"           => "String".into(),
        "xs:ENTITY"           => "String".into(),
        "xs:ID"               => "String".into(),
        "xs:IDREF"            => "String".into(),
        "xs:NMTOKEN"          => "String".into(),
        "xs:anyURI"           => "String".into(),
        "xs:QName"            => "String".into(),

        "xs::NOTATION" => "String".into(),

        // Built-in list types:
        "xs:ENTITIES" => "Vec<String>".into(),
        "xs:IDREFS"   => "Vec<String>".into(),
        "xs:NMTOKENS" => "Vec<String>".into(),

        x => {
            let prefix = target_namespace.and_then(|ns| ns.name());
            to_pascal_case(
                match prefix {
                    Some(name) => {
                        if x.starts_with(name) {
                            x[name.len() + 1..].to_string()
                        }
                        else {
                            x.replace(":", "::")
                        }
                    }
                    None => x.replace(":", "::")
                }.as_str()
            ).into()
        }
    }
}

pub fn get_field_name(name: &str) -> String {
    to_snake_case(name)
}

pub fn attribute_type(attr: &Attribute, type_name: Cow<str>) -> String {
    match attr.use_type() {
        UseType::Required => type_name.to_string(),
        UseType::Optional => format!("Option<{}>", type_name),
        UseType::Prohibited => format!("Empty<{}>", type_name),
    }
}
