use crate::utils;
use chrono::{NaiveDate, format::ParseError, format::strftime::StrftimeItems};
use std::fmt;
use std::io::{Read, Write};
use std::str::FromStr;
use yaserde::{YaDeserialize, YaSerialize};

// Note:
// time zones are not supported in current implementation.
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Date {
    pub value: NaiveDate,
}

impl Date {
    pub fn from_chrono_naive_date(date: NaiveDate) -> Self {
        Date { value: date }
    }

    pub fn to_chrono_naive_date(&self) -> NaiveDate {
        self.value
    }
}

impl Default for Date {
  fn default() -> Date {
    Self{ value: NaiveDate::from_ymd(1, 1, 1) }
  }
}

impl FromStr for Date {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Date {
            value: NaiveDate::parse_from_str(s, "%Y-%m-%d")?,
        })
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt = StrftimeItems::new("%Y-%m-%d");
        write!(f, "{}", self.value.format_with_items(fmt.clone()))
    }
}

impl YaDeserialize for Date {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        utils::yaserde::deserialize(reader, |s| Date::from_str(s).map_err(|e| e.to_string()))
    }
}

impl YaSerialize for Date {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        utils::yaserde::serialize(self, "Date", writer, |s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespace = "t: test")]
    pub struct Message {
        #[yaserde(prefix = "t", rename = "CreatedAt")]
        pub created_at: Date,

        #[yaserde(prefix = "t", rename = "Text")]
        pub text: String,
    }

    #[test]
    fn datetime_serialize_test() {
        let expected = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2020-02-02</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m = Message {
            created_at: Date::from_chrono_naive_date(NaiveDate::from_ymd(2020, 2, 2)),
            text: "Hello world".to_string(),
        };
        let actual = yaserde::ser::to_string(&m).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn integer_deserialize_test() {
        let s = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2020-02-02</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m: Message = yaserde::de::from_str(&s).unwrap();
        assert_eq!(m.created_at.to_chrono_naive_date(), NaiveDate::from_ymd(2020, 2, 2));
        assert_eq!(m.text, "Hello world".to_string());
    }
}
