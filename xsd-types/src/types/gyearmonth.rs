use crate::types::gmonth::GMonth;
use crate::types::gyear::GYear;
use crate::types::utils::parse_timezone;

use chrono::FixedOffset;
use std::fmt;
use std::str::FromStr;
use xsd_macro_utils::UtilsDefaultSerde;

#[derive(PartialEq, Debug, UtilsDefaultSerde)]
pub struct GYearMonth {
    pub year: i32,
    pub month: i32,
    pub timezone: Option<FixedOffset>,
}

impl GYearMonth {
    pub fn new(year: i32, month: i32, timezone: Option<FixedOffset>) -> Result<Self, String> {
        if year == 0 {
            return Err("bad gYear format: year 0 occurred".to_string());
        }

        if !(1..=12).contains(&month) {
            return Err("Month value within GYearMonth should lie between 1 and 12".to_string());
        }

        Ok(GYearMonth {
            year,
            month,
            timezone,
        })
    }

    pub fn gyear(self) -> GYear {
        GYear {
            value: self.year,
            timezone: self.timezone,
        }
    }

    pub fn gmonth(self) -> GMonth {
        GMonth {
            value: self.month,
            timezone: self.timezone,
        }
    }
}

impl Default for GYearMonth {
    fn default() -> GYearMonth {
        Self {
            year: 1,
            month: 1,
            timezone: None,
        }
    }
}

impl FromStr for GYearMonth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('-') {
            let mut gyearmonth = parse_str_positive(s)?;
            gyearmonth.year *= -1;
            return Ok(gyearmonth);
        }
        parse_str_positive(s)
    }
}

fn parse_str_positive(s: &str) -> Result<GYearMonth, String> {
    fn parse_value(s: &str) -> Result<(i32, i32), String> {
        if s.matches('-').count() != 1 {
            return Err("bad gYearMonth format".to_string());
        }

        let idx: usize = s.match_indices('-').collect::<Vec<_>>()[0].0;
        let year_token = &s[..idx];
        let month_token = &s[idx + 1..];
        if year_token.len() < 4 || month_token.len() != 2 {
            return Err("bad gYearMonth format".to_string());
        }

        if !year_token.chars().all(|c| c.is_ascii_digit()) {
            return Err("bad year format within gYearMonth".to_string());
        }
        let year = year_token.parse::<i32>().map_err(|e| e.to_string())?;

        if !month_token.chars().all(|c| c.is_ascii_digit()) {
            return Err("bad month format within gYearMonth".to_string());
        }
        let month = month_token.parse::<i32>().map_err(|e| e.to_string())?;

        Ok((year, month))
    }

    if let Some(s) = s.strip_suffix('Z') {
        let (year, month) = parse_value(s)?;
        return GYearMonth::new(year, month, Some(FixedOffset::east_opt(0).unwrap()));
    }

    if s.contains('+') {
        if s.matches('+').count() > 1 {
            return Err("bad gMonthDay format".to_string());
        }

        let idx: usize = s.match_indices('+').collect::<Vec<_>>()[0].0;
        let value_token = &s[..idx];
        let tz_token = &s[idx..];
        let (year, month) = parse_value(value_token)?;
        return GYearMonth::new(year, month, Some(parse_timezone(tz_token)?));
    }

    if s.matches('-').count() == 2 {
        let idx: usize = s.match_indices('-').collect::<Vec<_>>()[1].0;
        let value_token = &s[..idx];
        let tz_token = &s[idx..];
        let (year, month) = parse_value(value_token)?;
        return GYearMonth::new(year, month, Some(parse_timezone(tz_token)?));
    }

    let (year, month) = parse_value(s)?;
    GYearMonth::new(year, month, None)
}

impl fmt::Display for GYearMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.year > 0 {
            match self.timezone {
                Some(tz) => write!(f, "{:04}-{:02}{}", self.year, self.month, tz),
                None => write!(f, "{:04}-{:02}", self.year, self.month),
            }
        } else {
            match self.timezone {
                Some(tz) => write!(f, "-{:04}-{:02}{}", -self.year, self.month, tz),
                None => write!(f, "-{:04}-{:02}", -self.year, self.month),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;
    use yaserde_derive::{YaDeserialize, YaSerialize};

    #[test]
    fn gyearmonth_parse_test() {
        // No timezone.
        assert_eq!(
            GYearMonth::from_str("2020-03"),
            Ok(GYearMonth {
                year: 2020,
                month: 3,
                timezone: None
            })
        );

        // Timezone "Z".
        assert_eq!(
            GYearMonth::from_str("2020-03Z"),
            Ok(GYearMonth {
                year: 2020,
                month: 3,
                timezone: Some(FixedOffset::east_opt(0).unwrap())
            })
        );

        // Positive offset.
        assert_eq!(
            GYearMonth::from_str("2020-03+06:30"),
            Ok(GYearMonth {
                year: 2020,
                month: 3,
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Negative offset.
        assert_eq!(
            GYearMonth::from_str("2020-03-06:30"),
            Ok(GYearMonth {
                year: 2020,
                month: 3,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Negative year.
        assert_eq!(
            GYearMonth::from_str("-0020-03-06:30"),
            Ok(GYearMonth {
                year: -20,
                month: 3,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Negative year with five digits.
        assert_eq!(
            GYearMonth::from_str("-20000-03-06:30"),
            Ok(GYearMonth {
                year: -20000,
                month: 3,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            })
        );

        // Invalid values.
        assert!(GYearMonth::from_str("01-03").is_err());
        assert!(GYearMonth::from_str("2000-1").is_err());
        assert!(GYearMonth::from_str("2000-13").is_err());
        assert!(GYearMonth::from_str("2000-00").is_err());
        assert!(GYearMonth::from_str("0000-03").is_err());
        assert!(GYearMonth::from_str("2000-+3").is_err());
        assert!(GYearMonth::from_str("-200-03").is_err());
        assert!(GYearMonth::from_str("+200-03").is_err());
        assert!(GYearMonth::from_str("++++-++").is_err());
    }

    #[test]
    fn gyearmonth_display_test() {
        // No timezone.
        assert_eq!(
            GYearMonth {
                year: 987,
                month: 6,
                timezone: None
            }
            .to_string(),
            "0987-06"
        );

        // Timezone +00:00.
        assert_eq!(
            GYearMonth {
                year: 987,
                month: 6,
                timezone: Some(FixedOffset::east_opt(0).unwrap())
            }
            .to_string(),
            "0987-06+00:00"
        );

        // Positive offset.
        assert_eq!(
            GYearMonth {
                year: 987,
                month: 6,
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap())
            }
            .to_string(),
            "0987-06+06:30"
        );

        // Negative offset.
        assert_eq!(
            GYearMonth {
                year: 987,
                month: 6,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            }
            .to_string(),
            "0987-06-06:30"
        );

        // Negative year.
        assert_eq!(
            GYearMonth {
                year: -987,
                month: 6,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            }
            .to_string(),
            "-0987-06-06:30"
        );

        // Negative year with five digits.
        assert_eq!(
            GYearMonth {
                year: -98765,
                month: 6,
                timezone: Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
            }
            .to_string(),
            "-98765-06-06:30"
        );
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespace = "t: test")]
    pub struct Message {
        #[yaserde(prefix = "t", rename = "CreatedAt")]
        pub created_at: GYearMonth,

        #[yaserde(prefix = "t", rename = "Text")]
        pub text: String,
    }

    #[test]
    fn gyearmonth_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="utf-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2007-02+06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m = Message {
            created_at: GYearMonth {
                year: 2007,
                month: 2,
                timezone: Some(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap()),
            },
            text: "Hello world".to_string(),
        };
        let actual = yaserde::ser::to_string(&m).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn gyearmonth_deserialize_test() {
        let s = r#"<?xml version="1.0" encoding="utf-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2007-02-06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m: Message = yaserde::de::from_str(s).unwrap();
        assert_eq!(m.created_at.year, 2007);
        assert_eq!(m.created_at.month, 2);
        assert_eq!(
            m.created_at.timezone,
            Some(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap()),
        );
        assert_eq!(m.text, "Hello world".to_string());
    }
}
