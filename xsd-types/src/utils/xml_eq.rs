use xml::reader::{Error as XmlError, XmlEvent};

pub fn assert_xml_eq(actual: &str, expected: &str) {
    for (a, e) in without_whitespaces(actual).zip(without_whitespaces(expected)) {
        assert_eq!(a, e);
    }
}

fn without_whitespaces(expected: &str) -> impl Iterator<Item = Result<XmlEvent, XmlError>> + '_ {
    xml::EventReader::new(expected.as_bytes())
        .into_iter()
        .filter(|e| !matches!(e, Ok(XmlEvent::Whitespace(_))))
}
