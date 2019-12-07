extern crate inflector;
use inflector::cases::pascalcase::to_pascal_case;


pub fn get_comment(doc: Option<&str>) -> String {
    doc.
        unwrap_or("").
        lines().
        map(|s| s.trim()).
        filter(|s| s.len() > 2).
        map(|s| format!("// {}\n", s)).
        fold(String::new(), |x , y| (x+&y))
}

pub fn get_type_name(name: &str) -> String {
    to_pascal_case(name)
}