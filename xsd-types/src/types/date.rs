use crate::utils;
use chrono::{NaiveDate, format::strftime::StrftimeItems, FixedOffset};
use std::fmt;
use std::io::{Read, Write};
use std::str::FromStr;
use yaserde::{YaDeserialize, YaSerialize};

#[derive(PartialEq, Debug)]
pub struct Date {
    pub value: NaiveDate,
    pub timezone: Option<FixedOffset>,
}

impl Date {
    pub fn from_chrono_naive_date(date: NaiveDate) -> Self {
        Date { value: date, timezone: None }
    }

    pub fn to_chrono_naive_date(&self) -> NaiveDate {
        self.value
    }
}

impl Default for Date {
  fn default() -> Date {
    Self{ value: NaiveDate::from_ymd(1, 1, 1), timezone: None }
  }
}

impl FromStr for Date {
    type Err = String;

    // TODO: check timezone overflow (use east_opt)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("Z") {
            return match NaiveDate::parse_from_str(&s[..s.len()-1], "%Y-%m-%d") {
                Err(e) => Err(e.to_string()),
                Ok(d) => Ok(Date {
                    value: d,
                    timezone: Some(FixedOffset::east(0))
                })
            };
        }

        if s.contains("+") {
            let tokens: Vec<&str>= s.split("+").collect();
            if tokens.len() > 2 {
                return Err("bad date format".to_string());
            }

            let tz_tokens: Vec<&str>= tokens[1].split(":").collect();
            if tz_tokens.len() != 2 {
                return Err("bad timezone format".to_string());
            }

            let offset = 60 * tz_tokens[0].parse::<i32>().unwrap() + tz_tokens[1].parse::<i32>().unwrap();
            return match NaiveDate::parse_from_str(tokens[0], "%Y-%m-%d") {
                Err(e) => Err(e.to_string()),
                Ok(d) => Ok(Date {
                    value: d,
                    timezone: Some(FixedOffset::east(60 * offset))
                })
            };
        }

        if s.matches("-").count() == 3 {
            let idx: usize = s.match_indices("-").collect::<Vec<_>>()[2].0;
            let date_token = &s[..idx];
            let tz_tokens: Vec<&str>= (&s[idx+1..]).split(":").collect();

            if tz_tokens.len() != 2 {
                return Err("bad timezone format".to_string());
            }

            let offset = 60 * tz_tokens[0].parse::<i32>().unwrap() + tz_tokens[1].parse::<i32>().unwrap();
            return match NaiveDate::parse_from_str(date_token, "%Y-%m-%d") {
                Err(e) => Err(e.to_string()),
                Ok(d) => Ok(Date {
                    value: d,
                    timezone: Some(FixedOffset::east(60 * -offset))
                })
            };
        }

        return match NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Err(e) => Err(e.to_string()),
            Ok(d) => Ok(Date {
                value: d,
                timezone: None
            })
        };
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

    #[test]
    fn datetime_parse_test() {
        // No timezone.
        assert_eq!(Date::from_str("2020-02-02"), Ok(Date{ value: NaiveDate::from_ymd(2020, 2, 2), timezone: None }));

        // Timezone "Z".
        assert_eq!(Date::from_str("2020-02-02Z"), Ok(Date{ value: NaiveDate::from_ymd(2020, 2, 2), timezone: Some(FixedOffset::east(0)) }));

        // Positive offset.
        assert_eq!(Date::from_str("2020-02-02+06:30"), Ok(Date{ value: NaiveDate::from_ymd(2020, 2, 2), timezone: Some(FixedOffset::east(6 * 3600 + 30 * 60)) }));

        // Negative offset.
        assert_eq!(Date::from_str("2020-02-02-06:30"), Ok(Date{ value: NaiveDate::from_ymd(2020, 2, 2), timezone: Some(FixedOffset::west(6 * 3600 + 30 * 60)) }));
    }

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
