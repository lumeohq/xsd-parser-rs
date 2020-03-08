use crate::utils;
use chrono::{NaiveTime, format::ParseError, format::strftime::StrftimeItems};
use std::fmt;
use std::io::{Read, Write};
use std::str::FromStr;
use yaserde::{YaDeserialize, YaSerialize};

// Note:
// time zones are not supported in current implementation.
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Time {
    pub value: NaiveTime,
}

impl Time {
    pub fn from_chrono_naive_time(time: NaiveTime) -> Self {
        Time { value: time }
    }

    pub fn to_chrono_naive_time(&self) -> NaiveTime {
        self.value
    }
}

impl Default for Time {
  fn default() -> Time {
    Self{ value: NaiveTime::from_hms(0, 0, 0) }
  }
}

impl FromStr for Time {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Time {
            value: NaiveTime::parse_from_str(s, "%H:%M:%S")?,
        })
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt = StrftimeItems::new("%H:%M:%S");
        write!(f, "{}", self.value.format_with_items(fmt.clone()))
    }
}

impl YaDeserialize for Time {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        utils::yaserde::deserialize(reader, |s| Time::from_str(s).map_err(|e| e.to_string()))
    }
}

impl YaSerialize for Time {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        utils::yaserde::serialize(self, "Time", writer, |s| s.to_string())
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
        pub created_at: Time,

        #[yaserde(prefix = "t", rename = "Text")]
        pub text: String,
    }

    #[test]
    fn datetime_serialize_test() {
        let expected = r#"
            <?xml version="1.0" encoding="utf-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>04:40:00</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m = Message {
            created_at: Time::from_chrono_naive_time(NaiveTime::from_hms(4, 40, 0)),
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
                <t:CreatedAt>04:40:00</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m: Message = yaserde::de::from_str(&s).unwrap();
        assert_eq!(m.created_at.to_chrono_naive_time(), NaiveTime::from_hms(4, 40, 0));
        assert_eq!(m.text, "Hello world".to_string());
    }
}
