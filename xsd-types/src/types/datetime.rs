use crate::utils;
use chrono::{DateTime as CDateTime, FixedOffset};
use std::fmt;
use std::io::{Read, Write};
use std::str::FromStr;
use yaserde::{YaDeserialize, YaSerialize};

#[derive(PartialEq, PartialOrd, Debug)]
pub struct DateTime {
    pub value: CDateTime<FixedOffset>,
}

impl DateTime {
    pub fn from_chrono_datetime(datetime: CDateTime<FixedOffset>) -> Self {
        DateTime { value: datetime }
    }

    pub fn to_chrono_datetime(&self) -> CDateTime<FixedOffset> {
        self.value
    }
}

impl Default for DateTime {
  fn default() -> DateTime {
    Self{value: CDateTime::parse_from_rfc3339("0001-01-01T00:00:00Z").unwrap()}
  }
}

impl FromStr for DateTime {
    type Err = chrono::format::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DateTime {
            value: CDateTime::parse_from_rfc3339(s)?,
        })
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.to_rfc3339())
    }
}

impl YaDeserialize for DateTime {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        utils::yaserde::deserialize(reader, |s| DateTime::from_str(s).map_err(|e| e.to_string()))
    }
}

impl YaSerialize for DateTime {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        utils::yaserde::serialize(self, "DateTime", writer, |s| s.to_string())
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
        pub created_at: DateTime,

        #[yaserde(prefix = "t", rename = "Text")]
        pub text: String,
    }

    #[test]
    fn datetime_serialize_test() {
        let expected = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2020-03-07T04:40:00+00:00</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let i = Message {
            created_at: DateTime::from_str("2020-03-07T04:40:00Z").unwrap(),
            text: "Hello world".to_string(),
        };
        let actual = yaserde::ser::to_string(&i).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn integer_deserialize_test() {
        let s = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2020-03-07T04:40:00+00:00</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m: Message = yaserde::de::from_str(&s).unwrap();
        assert_eq!(m.created_at.to_chrono_datetime(), CDateTime::parse_from_rfc3339("2020-03-07T04:40:00Z").unwrap());
        assert_eq!(m.text, "Hello world".to_string());
    }
}
