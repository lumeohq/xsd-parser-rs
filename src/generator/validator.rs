use crate::parser::xsd_elements::FacetType;
use std::borrow::Cow;

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

pub trait ValidateGenerator {}

pub fn gen_validate_impl(name: &str, body: &str) -> String {
    format!(
        r#"impl Validate for {name} {{
    fn validate(&self) -> Result<(), String> {{
        {body}
        Ok(())
    }}
}}
"#,
        name = name,
        body = body
    )
}

pub fn gen_facet_validation(facet: &FacetType, name: &str) -> Cow<'static, str> {
    match facet {
        FacetType::Enumeration(_) => "".into(),
        FacetType::FractionDigits(_) => "".into(),
        FacetType::Length(value) => gen_length_validation(*value, name).into(),
        FacetType::MaxExclusive(value) => gen_max_exclusive_validation(value.as_str(), name).into(),
        FacetType::MaxInclusive(value) => gen_max_inclusive_validation(value.as_str(), name).into(),
        FacetType::MaxLength(value) => gen_max_length_validation(*value, name).into(),
        FacetType::MinExclusive(value) => gen_min_exclusive_validation(value.as_str(), name).into(),
        FacetType::MinInclusive(value) => gen_min_inclusive_validation(value.as_str(), name).into(),
        FacetType::MinLength(value) => gen_min_length_validation(*value, name).into(),
        _ => "".into(), // TODO: All Facet Types
    }
}

fn gen_max_exclusive_validation(value: &str, name: &str) -> String {
    format!(
        r#"
        if self.{name} >= {value} {{
            return Err(format!("MaxExclusive validation error: invalid value of {name}! \nExpected: {name} < {value}.\nActual: {name} == {{}}", self.{name}));
        }}"#,
        name = name,
        value = value
    )
}

fn gen_max_inclusive_validation(value: &str, name: &str) -> String {
    format!(
        r#"
        if self.{name} > {value} {{
            return Err(format!("MaxInclusive validation error: invalid value of {name}! \nExpected: {name} <= {value}.\nActual: {name} == {{}}", self.{name}));
        }}"#,
        name = name,
        value = value
    )
}

fn gen_length_validation(value: usize, name: &str) -> String {
    format!(
        r#"
        if self.{name}.len() != {value} {{
            return Err(format!("Length validation error. \nExpected: {name} length == {value} \nActual: {name} length == {{}}", self.{name}.len()));
        }}"#,
        name = name,
        value = value
    )
}

fn gen_max_length_validation(value: usize, name: &str) -> String {
    format!(
        r#"
        if self.{name}.len() > {value} {{
            return Err(format!("MaxLength validation error. \nExpected: {name} length <= {value} \nActual: {name} length == {{}}", self.{name}.len()));
        }}"#,
        name = name,
        value = value
    )
}

fn gen_min_exclusive_validation(value: &str, name: &str) -> String {
    format!(
        r#"
        if self.{name} <= {value} {{
            return Err(format!("MinExclusive validation error: invalid value of {name}! \nExpected: {name} > {value}.\nActual: {name} == {{}}", self.{name}));
        }}"#,
        name = name,
        value = value
    )
}

fn gen_min_inclusive_validation(value: &str, name: &str) -> String {
    format!(
        r#"
        if self.{name} < {value} {{
            return Err(format!("MinInclusive validation error: invalid value of {name}! \nExpected: {name} >= {value}.\nActual: {name} == {{}}", self.{name}));
        }}"#,
        name = name,
        value = value
    )
}

fn gen_min_length_validation(value: usize, name: &str) -> String {
    format!(
        r#"
        if self.{name}.len() < {value} {{
            return Err(format!("MinLength validation error. \nExpected: {name} length >= {value} \nActual: {name} length == {{}}", self.{name}.len()));
        }}"#,
        name = name,
        value = value
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validator_for_tuple_struct() {
        struct Foo(i64);
        impl Validate for Foo {
            fn validate(&self) -> Result<(), String> {
                Err("Error".to_owned())
            }
        }
    }

    #[test]
    fn test_gen_max_exclusive_validation() {
        let expected = r#"
        if self.count >= 5 {
            return Err(format!("MaxExclusive validation error: invalid value of count! \nExpected: count < 5.\nActual: count == {}", self.count));
        }"#;
        assert_eq!(gen_max_exclusive_validation("5", "count"), expected);
    }

    #[test]
    fn test_gen_max_inclusive_validation() {
        let expected = r#"
        if self.count > 5 {
            return Err(format!("MaxInclusive validation error: invalid value of count! \nExpected: count <= 5.\nActual: count == {}", self.count));
        }"#;
        assert_eq!(gen_max_inclusive_validation("5", "count"), expected);
    }

    #[test]
    fn test_gen_length_validation() {
        let expected = r#"
        if self.name.len() != 50 {
            return Err(format!("Length validation error. \nExpected: name length == 50 \nActual: name length == {}", self.name.len()));
        }"#;
        assert_eq!(gen_length_validation(50, "name"), expected);
    }

    #[test]
    fn test_gen_max_length_validation() {
        let expected = r#"
        if self.name.len() > 50 {
            return Err(format!("MaxLength validation error. \nExpected: name length <= 50 \nActual: name length == {}", self.name.len()));
        }"#;
        assert_eq!(gen_max_length_validation(50, "name"), expected);
    }

    #[test]
    fn test_gen_min_exclusive_validation() {
        let expected = r#"
        if self.count <= 5 {
            return Err(format!("MinExclusive validation error: invalid value of count! \nExpected: count > 5.\nActual: count == {}", self.count));
        }"#;
        assert_eq!(gen_min_exclusive_validation("5", "count"), expected);
    }

    #[test]
    fn test_gen_min_inclusive_validation() {
        let expected = r#"
        if self.count < 5 {
            return Err(format!("MinInclusive validation error: invalid value of count! \nExpected: count >= 5.\nActual: count == {}", self.count));
        }"#;
        assert_eq!(gen_min_inclusive_validation("5", "count"), expected);
    }

    #[test]
    fn test_gen_min_length_validation() {
        let expected = r#"
        if self.name.len() < 50 {
            return Err(format!("MinLength validation error. \nExpected: name length >= 50 \nActual: name length == {}", self.name.len()));
        }"#;
        assert_eq!(gen_min_length_validation(50, "name"), expected);
    }
}
