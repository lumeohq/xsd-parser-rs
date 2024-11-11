use std::{fmt, str::FromStr};

use chrono::{format::strftime::StrftimeItems, FixedOffset, NaiveDate};
use xsd_macro_utils::UtilsDefaultSerde;

use crate::types::utils::parse_timezone;

#[derive(PartialEq, Debug, Clone, UtilsDefaultSerde)]
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
        Self { value: NaiveDate::from_ymd_opt(1, 1, 1).unwrap(), timezone: None }
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_naive_date(s: &str) -> Result<NaiveDate, String> {
            NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| e.to_string())
        }

        if let Some(s) = s.strip_suffix('Z') {
            return Ok(Date {
                value: parse_naive_date(s)?,
                timezone: Some(FixedOffset::east_opt(0).unwrap()),
            });
        }

        if s.contains('+') {
            if s.matches('+').count() > 1 {
                return Err("bad date format".to_string());
            }

            let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
            let date_token = &s[..idx];
            let tz_token = &s[idx..];
            return Ok(Date {
                value: parse_naive_date(date_token)?,
                timezone: Some(parse_timezone(tz_token)?),
            });
        }

        if s.matches('-').count() == 3 {
            let idx: usize = s.match_indices('-').collect::<Vec<_>>()[2].0;
            let date_token = &s[..idx];
            let tz_token = &s[idx..];
            return Ok(Date {
                value: parse_naive_date(date_token)?,
                timezone: Some(parse_timezone(tz_token)?),
            });
        }

        Ok(Date { value: parse_naive_date(s)?, timezone: None })
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt = StrftimeItems::new("%Y-%m-%d");
        match self.timezone {
            Some(tz) => write!(f, "{}{}", self.value.format_with_items(fmt.clone()), tz),
            None => write!(f, "{}", self.value.format_with_items(fmt.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;

    #[test]
    fn date_parse_test() {
        // No timezone.
        assert_eq!(
            Date::from_str("2020-02-02"),
            Ok(Date { value: NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(), timezone: None })
        );

        // Timezone "Z".
        assert_eq!(
            Date::from_str("2020-02-02Z"),
            Ok(Date {
                value: NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(),
                timezone: Some(FixedOffset::east_opt(0).unwrap())
            })
        );

        // Positive offset.
        assert_eq!(
            Date::from_str("2020-02-02+06:30"),
            Ok(Date {
                value: NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(),
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Negative offset.
        assert_eq!(
            Date::from_str("2020-02-02-06:30"),
            Ok(Date {
                value: NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(),
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );
    }

    #[test]
    fn date_display_test() {
        // No timezone.
        assert_eq!(
            Date { value: NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(), timezone: None }
                .to_string(),
            "2020-02-02"
        );

        // Timezone +00:00.
        assert_eq!(
            Date {
                value: NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(),
                timezone: Some(FixedOffset::east_opt(0).unwrap())
            }
            .to_string(),
            "2020-02-02+00:00"
        );

        // Positive offset.
        assert_eq!(
            Date {
                value: NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(),
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap())
            }
            .to_string(),
            "2020-02-02+06:30"
        );

        // Negative offset.
        assert_eq!(
            Date {
                value: NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(),
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            }
            .to_string(),
            "2020-02-02-06:30"
        );
    }

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespaces = {"t" = "test"})]
    pub struct Message {
        #[yaserde(prefix = "t", rename = "CreatedAt")]
        pub created_at: Date,

        #[yaserde(prefix = "t", rename = "Text")]
        pub text: String,
    }

    #[test]
    fn date_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2020-02-02+06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m = Message {
            created_at: Date {
                value: NaiveDate::from_ymd_opt(2020, 2, 2).unwrap(),
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap()),
            },
            text: "Hello world".to_string(),
        };
        let actual = yaserde::ser::to_string(&m).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn date_deserialize_test() {
        let s = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2020-02-02-06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m: Message = yaserde::de::from_str(s).unwrap();
        assert_eq!(m.created_at.value, NaiveDate::from_ymd_opt(2020, 2, 2).unwrap());
        assert_eq!(m.created_at.timezone, Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap()),);
        assert_eq!(m.text, "Hello world".to_string());
    }
}
