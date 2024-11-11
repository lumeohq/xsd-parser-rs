use std::{fmt, str::FromStr};

use chrono::{format::ParseError, DateTime as CDateTime, FixedOffset};
use xsd_macro_utils::UtilsDefaultSerde;

use crate::types::datetime::DateTime;

// The only difference from DateTime is that the time zone expression is required at the end of the value.
#[derive(Default, Clone, PartialEq, PartialOrd, Debug, UtilsDefaultSerde)]
pub struct DateTimeStamp {
    pub value: DateTime,
}

impl DateTimeStamp {
    pub fn from_chrono_datetime(datetime: CDateTime<FixedOffset>) -> Self {
        DateTimeStamp { value: DateTime::from_chrono_datetime(datetime) }
    }

    pub fn to_chrono_datetime(&self) -> CDateTime<FixedOffset> {
        self.value.to_chrono_datetime()
    }
}

impl FromStr for DateTimeStamp {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match CDateTime::parse_from_rfc3339(s) {
            Ok(cdt) => Ok(DateTimeStamp::from_chrono_datetime(cdt)),
            Err(err) => Err(err),
        }
    }
}

impl fmt::Display for DateTimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
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
        assert!(DateTimeStamp::from_str("2020-03-07T04:40:00").is_err());
        // Timezone "Z".
        assert_eq!(
            DateTimeStamp::from_str("2020-03-07T04:40:00Z"),
            Ok(DateTimeStamp::from_chrono_datetime(dt))
        );

        // Positive offset.
        let offset = FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(
            DateTimeStamp::from_str("2020-03-07T04:40:00+06:30"),
            Ok(DateTimeStamp::from_chrono_datetime(dt))
        );

        // Negative offset.
        let offset = FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(
            DateTimeStamp::from_str("2020-03-07T04:40:00-06:30"),
            Ok(DateTimeStamp::from_chrono_datetime(dt))
        );
    }

    #[test]
    fn datetime_display_test() {
        // Timezone +00:00.
        let offset = FixedOffset::east_opt(0).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(
            DateTimeStamp::from_chrono_datetime(dt).to_string(),
            "2020-03-07T04:40:00+00:00"
        );

        // Positive offset.
        let offset = FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(
            DateTimeStamp::from_chrono_datetime(dt).to_string(),
            "2020-03-07T04:40:00+06:30"
        );

        // Negative offset.
        let offset = FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap();
        let dt_utc =
            NaiveDate::from_ymd_opt(2020, 3, 7).unwrap().and_hms_opt(4, 40, 0).unwrap() - offset;
        let dt = CDateTime::<FixedOffset>::from_naive_utc_and_offset(dt_utc, offset);
        assert_eq!(
            DateTimeStamp::from_chrono_datetime(dt).to_string(),
            "2020-03-07T04:40:00-06:30"
        );
    }

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespaces = {"t" = "test"})]
    pub struct Message {
        #[yaserde(prefix = "t", rename = "CreatedAt")]
        pub created_at: DateTimeStamp,

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
        let m = Message {
            created_at: DateTimeStamp::from_chrono_datetime(dt),
            text: "Hello world".to_string(),
        };
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

        assert_eq!(m.created_at.value, DateTime::from_chrono_datetime(dt));
        assert_eq!(m.text, "Hello world".to_string());
    }
}
