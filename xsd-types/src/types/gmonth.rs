use std::{fmt, str::FromStr};

use chrono::FixedOffset;
use xsd_macro_utils::UtilsDefaultSerde;

use crate::types::utils::parse_timezone;

#[derive(PartialEq, Debug, Clone, UtilsDefaultSerde)]
pub struct GMonth {
    pub value: i32,
    pub timezone: Option<FixedOffset>,
}

impl GMonth {
    pub fn new(month: i32, timezone: Option<FixedOffset>) -> Result<Self, String> {
        if !(1..=12).contains(&month) {
            return Err("GMonth value should lie between 1 and 12".to_string());
        }
        Ok(GMonth { value: month, timezone })
    }
}

impl Default for GMonth {
    fn default() -> GMonth {
        Self { value: 1, timezone: None }
    }
}

impl FromStr for GMonth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_value(s: &str) -> Result<i32, String> {
            if s.len() != 4 || &s[0..2] != "--" {
                return Err("bad gMonth format".to_string());
            }
            let token = &s[2..4];
            if !token.chars().all(|c| c.is_ascii_digit()) {
                return Err("bad gMonth format".to_string());
            }
            token.parse::<i32>().map_err(|e| e.to_string())
        }

        if let Some(s) = s.strip_suffix('Z') {
            return GMonth::new(parse_value(s)?, Some(FixedOffset::east_opt(0).unwrap()));
        }

        if s.contains('+') {
            if s.matches('+').count() > 1 {
                return Err("bad gMonth format".to_string());
            }

            let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
            let value_token = &s[..idx];
            let tz_token = &s[idx..];
            return GMonth::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
        }

        if s.matches('-').count() == 3 {
            let idx: usize = s.match_indices('-').collect::<Vec<_>>()[2].0;
            let value_token = &s[..idx];
            let tz_token = &s[idx..];
            return GMonth::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
        }

        GMonth::new(parse_value(s)?, None)
    }
}

impl fmt::Display for GMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.timezone {
            Some(tz) => write!(f, "--{:02}{}", self.value, tz),
            None => write!(f, "--{:02}", self.value),
        }
    }
}

#[cfg(test)]
mod tests {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;

    #[test]
    fn gmonth_parse_test() {
        // No timezone.
        assert_eq!(GMonth::from_str("--12"), Ok(GMonth { value: 12, timezone: None }));

        // Timezone "Z".
        assert_eq!(
            GMonth::from_str("--12Z"),
            Ok(GMonth { value: 12, timezone: Some(FixedOffset::east_opt(0).unwrap()) })
        );

        // Positive offset.
        assert_eq!(
            GMonth::from_str("--12+06:30"),
            Ok(GMonth {
                value: 12,
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Negative offset.
        assert_eq!(
            GMonth::from_str("--12-06:30"),
            Ok(GMonth {
                value: 12,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Invalid values.
        assert!(GMonth::from_str("-10-").is_err());
        assert!(GMonth::from_str("--15").is_err());
        assert!(GMonth::from_str("--5").is_err());
        assert!(GMonth::from_str("11").is_err());
        assert!(GMonth::from_str("----11").is_err());
        assert!(GMonth::from_str("----1").is_err());
        assert!(GMonth::from_str("--+1").is_err());
    }

    #[test]
    fn gmonth_display_test() {
        // No timezone.
        assert_eq!(GMonth { value: 3, timezone: None }.to_string(), "--03");

        // Timezone +00:00.
        assert_eq!(
            GMonth { value: 3, timezone: Some(FixedOffset::east_opt(0).unwrap()) }.to_string(),
            "--03+00:00"
        );

        // Positive offset.
        assert_eq!(
            GMonth { value: 3, timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap()) }
                .to_string(),
            "--03+06:30"
        );

        // Negative offset.
        assert_eq!(
            GMonth { value: 3, timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap()) }
                .to_string(),
            "--03-06:30"
        );
    }

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespaces = {"t" = "test"})]
    pub struct Message {
        #[yaserde(prefix = "t", rename = "CreatedAt")]
        pub created_at: GMonth,

        #[yaserde(prefix = "t", rename = "Text")]
        pub text: String,
    }

    #[test]
    fn gmonth_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>--07+06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m = Message {
            created_at: GMonth {
                value: 7,
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap()),
            },
            text: "Hello world".to_string(),
        };
        let actual = yaserde::ser::to_string(&m).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn gmonth_deserialize_test() {
        let s = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>--09-06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m: Message = yaserde::de::from_str(s).unwrap();
        assert_eq!(m.created_at.value, 9);
        assert_eq!(m.created_at.timezone, Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap()),);
        assert_eq!(m.text, "Hello world".to_string());
    }
}
