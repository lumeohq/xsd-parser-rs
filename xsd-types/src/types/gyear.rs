use std::{fmt, str::FromStr};

use chrono::FixedOffset;
use xsd_macro_utils::UtilsDefaultSerde;

use crate::types::utils::parse_timezone;

#[derive(PartialEq, Debug, Clone, UtilsDefaultSerde)]
pub struct GYear {
    pub value: i32,
    pub timezone: Option<FixedOffset>,
}

impl GYear {
    pub fn new(year: i32, timezone: Option<FixedOffset>) -> Result<Self, String> {
        if year == 0 {
            return Err("bad gYear format: year 0 occurred".to_string());
        }
        Ok(GYear { value: year, timezone })
    }
}

impl Default for GYear {
    fn default() -> GYear {
        Self { value: 1, timezone: None }
    }
}

impl FromStr for GYear {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('-') {
            let mut gyear = parse_str_positive(s)?;
            gyear.value *= -1;
            return Ok(gyear);
        }
        parse_str_positive(s)
    }
}

fn parse_str_positive(s: &str) -> Result<GYear, String> {
    fn parse_value(s: &str) -> Result<i32, String> {
        if s.len() < 4 {
            return Err("bad gYear format: to short".to_string());
        }
        if !s.chars().all(|c| c.is_ascii_digit()) {
            return Err("bad gYear format".to_string());
        }
        s.parse::<i32>().map_err(|e| e.to_string())
    }

    if let Some(s) = s.strip_suffix('Z') {
        return GYear::new(parse_value(s)?, Some(FixedOffset::east_opt(0).unwrap()));
    }

    if s.contains('+') {
        if s.matches('+').count() > 1 {
            return Err("bad gYear format".to_string());
        }

        let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
        let value_token = &s[..idx];
        let tz_token = &s[idx..];
        return GYear::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
    }

    if s.contains('-') {
        if s.matches('-').count() > 1 {
            return Err("bad gYear format".to_string());
        }

        let idx: usize = s.match_indices('-').collect::<Vec<_>>()[0].0;
        let value_token = &s[..idx];
        let tz_token = &s[idx..];
        return GYear::new(parse_value(value_token)?, Some(parse_timezone(tz_token)?));
    }

    GYear::new(parse_value(s)?, None)
}

impl fmt::Display for GYear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value > 0 {
            match self.timezone {
                Some(tz) => write!(f, "{:04}{}", self.value, tz),
                None => write!(f, "{:04}", self.value),
            }
        } else {
            match self.timezone {
                Some(tz) => write!(f, "-{:04}{}", -self.value, tz),
                None => write!(f, "-{:04}", -self.value),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;

    #[test]
    fn gyear_parse_test() {
        // No timezone.
        assert_eq!(GYear::from_str("2020"), Ok(GYear { value: 2020, timezone: None }));

        // Timezone "Z".
        assert_eq!(
            GYear::from_str("2020Z"),
            Ok(GYear { value: 2020, timezone: Some(FixedOffset::east_opt(0).unwrap()) })
        );

        // Positive offset.
        assert_eq!(
            GYear::from_str("2020+06:30"),
            Ok(GYear {
                value: 2020,
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Negative offset.
        assert_eq!(
            GYear::from_str("2020-06:30"),
            Ok(GYear {
                value: 2020,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Negative year.
        assert_eq!(
            GYear::from_str("-0020-06:30"),
            Ok(GYear {
                value: -20,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Negative year with five digits.
        assert_eq!(
            GYear::from_str("-20000-06:30"),
            Ok(GYear {
                value: -20000,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Invalid values.
        assert!(GYear::from_str("01").is_err());
        assert!(GYear::from_str("2001-12").is_err());
        assert!(GYear::from_str("0000").is_err());
        assert!(GYear::from_str("+123").is_err());
    }

    #[test]
    fn gyear_display_test() {
        // No timezone.
        assert_eq!(GYear { value: 987, timezone: None }.to_string(), "0987");

        // Timezone +00:00.
        assert_eq!(
            GYear { value: 987, timezone: Some(FixedOffset::east_opt(0).unwrap()) }.to_string(),
            "0987+00:00"
        );

        // Positive offset.
        assert_eq!(
            GYear {
                value: 987,
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap())
            }
            .to_string(),
            "0987+06:30"
        );

        // Negative offset.
        assert_eq!(
            GYear {
                value: 987,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            }
            .to_string(),
            "0987-06:30"
        );

        // Negative year.
        assert_eq!(
            GYear {
                value: -987,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            }
            .to_string(),
            "-0987-06:30"
        );

        // Negative year with five digits.
        assert_eq!(
            GYear {
                value: -98765,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            }
            .to_string(),
            "-98765-06:30"
        );
    }

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespaces = {"t" = "test"})]
    pub struct Message {
        #[yaserde(prefix = "t", rename = "CreatedAt")]
        pub created_at: GYear,

        #[yaserde(prefix = "t", rename = "Text")]
        pub text: String,
    }

    #[test]
    fn gyear_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2007+06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m = Message {
            created_at: GYear {
                value: 2007,
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap()),
            },
            text: "Hello world".to_string(),
        };
        let actual = yaserde::ser::to_string(&m).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn gyear_deserialize_test() {
        let s = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2007-06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m: Message = yaserde::de::from_str(s).unwrap();
        assert_eq!(m.created_at.value, 2007);
        assert_eq!(m.created_at.timezone, Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap()),);
        assert_eq!(m.text, "Hello world".to_string());
    }
}
