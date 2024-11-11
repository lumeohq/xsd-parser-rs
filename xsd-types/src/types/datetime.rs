use std::{fmt, str::FromStr};

use chrono::{format::ParseError, DateTime as CDateTime, FixedOffset};
use xsd_macro_utils::UtilsDefaultSerde;

#[derive(PartialEq, PartialOrd, Debug, Clone, UtilsDefaultSerde)]
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
        Self { value: CDateTime::parse_from_rfc3339("0001-01-01T00:00:00Z").unwrap() }
    }
}

impl FromStr for DateTime {
    type Err = ParseError;

    // Note:
    // `parse_from_rfc3339` parses an RFC 3339 and ISO 8601 date and time string.
    // XSD follows ISO 8601, which allows no time zone at the end of literal.
    // Since RFC 3339 does not allow such behavior, the function tries to add
    // 'Z' (which equals "+00:00") in case there is no timezone provided.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tz_provided = s.ends_with('Z') || s.contains('+') || s.matches('-').count() == 3;
        let s_with_timezone = if tz_provided { s.to_string() } else { format!("{}Z", s) };
        match CDateTime::parse_from_rfc3339(&s_with_timezone) {
            Ok(cdt) => Ok(DateTime { value: cdt }),
            Err(err) => Err(err),
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.to_rfc3339())
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;

    #[test]
    fn datetime_parse_test() {
        // No timezone.
        let offset = FixedOffset::east_opt(0).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(DateTime::from_str("2020-03-07T04:40:00"), Ok(DateTime { value: dt }));
        // Timezone "Z".
        assert_eq!(DateTime::from_str("2020-03-07T04:40:00Z"), Ok(DateTime { value: dt }));

        // Positive offset.
        let offset = FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(DateTime::from_str("2020-03-07T04:40:00+06:30"), Ok(DateTime { value: dt }));

        // Negative offset.
        let offset = FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(DateTime::from_str("2020-03-07T04:40:00-06:30"), Ok(DateTime { value: dt }));
    }

    #[test]
    fn datetime_display_test() {
        // Timezone +00:00.
        let offset = FixedOffset::east_opt(0).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(DateTime { value: dt }.to_string(), "2020-03-07T04:40:00+00:00");

        // Positive offset.
        let offset = FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(DateTime { value: dt }.to_string(), "2020-03-07T04:40:00+06:30");

        // Negative offset.
        let offset = FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(DateTime { value: dt }.to_string(), "2020-03-07T04:40:00-06:30");
    }

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespaces = {"t" = "test"})]
    pub struct Message {
        #[yaserde(prefix = "t", rename = "CreatedAt")]
        pub created_at: DateTime,

        #[yaserde(prefix = "t", rename = "Text")]
        pub text: String,
    }

    #[test]
    fn datetime_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2020-03-07T04:40:00+06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;

        let offset = FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        let m = Message { created_at: DateTime { value: dt }, text: "Hello world".to_string() };
        let actual = yaserde::ser::to_string(&m).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn datetime_deserialize_test() {
        let s = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:Message xmlns:t="test">
                <t:CreatedAt>2020-03-07T04:40:00-06:30</t:CreatedAt>
                <t:Text>Hello world</t:Text>
            </t:Message>
            "#;
        let m: Message = yaserde::de::from_str(s).unwrap();

        let offset = FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);

        assert_eq!(m.created_at.value, dt);
        assert_eq!(m.text, "Hello world".to_string());
    }
}
