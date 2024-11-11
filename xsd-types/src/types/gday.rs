use std::{fmt, str::FromStr};

use chrono::FixedOffset;
use xsd_macro_utils::UtilsDefaultSerde;

use crate::types::utils::parse_timezone;

#[derive(PartialEq, Debug, Clone, UtilsDefaultSerde)]
pub struct GDay {
    pub value: i32,
    pub timezone: Option<FixedOffset>,
}

impl GDay {
    pub fn new(day: i32, timezone: Option<FixedOffset>) -> Result<Self, String> {
        if !(1..=31).contains(&day) {
            return Err("gDay value should lie between 1 and 31".to_string());
        }
        Ok(GDay { value: day, timezone })
    }
}

impl Default for GDay {
    fn default() -> GDay {
        Self { value: 1, timezone: None }
    }
}

impl FromStr for GDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_value(s: &str) -> Result<i32, String> {
            if s.len() != 5 || &s[0..3] != "---" {
                return Err("bad gDay format".to_string());
            }
            let token = &s[3..5];
            if !token.chars().all(|c| c.is_ascii_digit()) {
                return Err("bad gDay format".to_string());
            }
            token.parse::<i32>().map_err(|e| e.to_string())
        }

        if let Some(s) = s.strip_suffix('Z') {
            return GDay::new(parse_value(s)?, Some(FixedOffset::east_opt(0).unwrap()));
        }

        if s.contains('+') {
            if s.matches('+').count() > 1 {
                return Err("bad gDay format".to_string());
            }

            let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
            let value_token = &s[..idx];
            let tz_token = &s[idx..];
            return GDay::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
        }

        if s.matches('-').count() == 4 {
            let idx: usize = s.match_indices('-').collect::<Vec<_>>()[3].0;
            let value_token = &s[..idx];
            let tz_token = &s[idx..];
            return GDay::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
        }

        GDay::new(parse_value(s)?, None)
    }
}

impl fmt::Display for GDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.timezone {
            Some(tz) => write!(f, "---{:02}{}", self.value, tz),
            None => write!(f, "---{:02}", self.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;

    #[test]
    fn gday_parse_test() {
        // No timezone.
        assert_eq!(GDay::from_str("---25"), Ok(GDay { value: 25, timezone: None }));

        // Timezone "Z".
        assert_eq!(
            GDay::from_str("---25Z"),
            Ok(GDay { value: 25, timezone: Some(FixedOffset::east_opt(0).unwrap()) })
        );

        // Positive offset.
        assert_eq!(
            GDay::from_str("---25+06:30"),
            Ok(GDay {
                value: 25,
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Negative offset.
        assert_eq!(
            GDay::from_str("---25-06:30"),
            Ok(GDay {
                value: 25,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Invalid values.
        assert!(GDay::from_str("--30-").is_err());
        assert!(GDay::from_str("---35").is_err());
        assert!(GDay::from_str("---5").is_err());
        assert!(GDay::from_str("15").is_err());
        assert!(GDay::from_str("----15").is_err());
        assert!(GDay::from_str("----5").is_err());
        assert!(GDay::from_str("---+5").is_err());
    }

    #[test]
    fn gday_display_test() {
        // No timezone.
        assert_eq!(GDay { value: 3, timezone: None }.to_string(), "---03");

        // Timezone +00:00.
        assert_eq!(
            GDay { value: 3, timezone: Some(FixedOffset::east_opt(0).unwrap()) }.to_string(),
            "---03+00:00"
        );

        // Positive offset.
        assert_eq!(
            GDay { value: 3, timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap()) }
                .to_string(),
            "---03+06:30"
        );

        // Negative offset.
        assert_eq!(
            GDay { value: 3, timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap()) }
                .to_string(),
            "---03-06:30"
        );
    }

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespaces = {"t" = "test"})]
    pub struct Message {
        #[yaserde(prefix = "t", rename = "CreatedAt")]
        pub created_at: GDay,

        #[yaserde(prefix = "t", rename = "Text")]
        pub text: String,
    }

    #[test]
    fn gday_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>---07+06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m = Message {
            created_at: GDay {
                value: 7,
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap()),
            },
            text: "Hello world".to_string(),
        };
        let actual = yaserde::ser::to_string(&m).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn gday_deserialize_test() {
        let s = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>---29-06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m: Message = yaserde::de::from_str(s).unwrap();
        assert_eq!(m.created_at.value, 29);
        assert_eq!(m.created_at.timezone, Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap()),);
        assert_eq!(m.text, "Hello world".to_string());
    }
}
