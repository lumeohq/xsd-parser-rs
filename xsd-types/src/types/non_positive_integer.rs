use std::{fmt, str::FromStr};

use num_bigint::{BigInt, ToBigInt};
use xsd_macro_utils::UtilsDefaultSerde;

// https://www.w3.org/TR/xmlschema-2/#nonPositiveInteger
#[derive(Default, Clone, PartialEq, PartialOrd, Debug, UtilsDefaultSerde)]
pub struct NonPositiveInteger(pub BigInt);

impl NonPositiveInteger {
    pub fn from_bigint(bigint: BigInt) -> Self {
        NonPositiveInteger(bigint)
    }
}

impl ToBigInt for NonPositiveInteger {
    fn to_bigint(&self) -> Option<BigInt> {
        Some(self.0.clone())
    }
}

impl FromStr for NonPositiveInteger {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = BigInt::from_str(s).map_err(|e| e.to_string())?;
        if value > 0.to_bigint().unwrap() {
            Err("Bad value for NonPositiveInteger".to_string())
        } else {
            Ok(NonPositiveInteger(value))
        }
    }
}

impl fmt::Display for NonPositiveInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_str_radix(10))
    }
}

#[cfg(test)]
mod tests {
    use yaserde::{YaDeserialize, YaSerialize};

    use super::*;
    use crate::utils::xml_eq::assert_xml_eq;

    #[test]
    fn non_positive_integer_parse_test() {
        assert_eq!(
            NonPositiveInteger::from_str("-12678967543233"),
            Ok(NonPositiveInteger(BigInt::from_str("-12678967543233").unwrap()))
        );

        assert_eq!(
            NonPositiveInteger::from_str("-100000"),
            Ok(NonPositiveInteger((-100000).to_bigint().unwrap()))
        );

        assert_eq!(
            NonPositiveInteger::from_str("-1"),
            Ok(NonPositiveInteger((-1).to_bigint().unwrap()))
        );

        assert_eq!(
            NonPositiveInteger::from_str("0"),
            Ok(NonPositiveInteger(0.to_bigint().unwrap()))
        );

        // Invalid values.
        assert!(NonPositiveInteger::from_str("1").is_err());
        assert!(NonPositiveInteger::from_str("+1234").is_err());
        assert!(NonPositiveInteger::from_str("A").is_err());
        assert!(NonPositiveInteger::from_str("--1").is_err());
        assert!(NonPositiveInteger::from_str("++1").is_err());
        assert!(NonPositiveInteger::from_str("-+1").is_err());
    }

    #[test]
    fn non_positive_integer_display_test() {
        assert_eq!(
            NonPositiveInteger(BigInt::from_str("-12678967543233").unwrap()).to_string(),
            "-12678967543233"
        );

        assert_eq!(NonPositiveInteger((-100000).to_bigint().unwrap()).to_string(), "-100000");

        assert_eq!(NonPositiveInteger(0.to_bigint().unwrap()).to_string(), "0");

        assert_eq!(NonPositiveInteger((-1).to_bigint().unwrap()).to_string(), "-1");
    }

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespaces = {"t" = "test"})]
    pub struct NonPositiveIntegerPair {
        #[yaserde(prefix = "t", rename = "First")]
        pub first: NonPositiveInteger,

        #[yaserde(prefix = "t", rename = "Second")]
        pub second: NonPositiveInteger,
    }

    #[test]
    fn non_positive_integer_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:NonPositiveIntegerPair xmlns:t="test">
                <t:First>0</t:First>
                <t:Second>-1234</t:Second>
            </t:NonPositiveIntegerPair>
            "#;
        let i = NonPositiveIntegerPair {
            first: NonPositiveInteger::from_bigint(0.to_bigint().unwrap()),
            second: NonPositiveInteger::from_bigint((-1234).to_bigint().unwrap()),
        };
        let actual = yaserde::ser::to_string(&i).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn non_positive_integer_deserialize_test() {
        let s = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:NonPositiveIntegerPair xmlns:t="test">
                <t:First>0</t:First>
                <t:Second>-1234</t:Second>
            </t:NonPositiveIntegerPair>
            "#;
        let i: NonPositiveIntegerPair = yaserde::de::from_str(s).unwrap();
        assert_eq!(i.first.to_bigint().unwrap(), 0.to_bigint().unwrap());
        assert_eq!(i.second.to_bigint().unwrap(), (-1234).to_bigint().unwrap());
    }
}
