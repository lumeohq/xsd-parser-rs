use std::{fmt, str::FromStr};

use num_bigint::{BigInt, ParseBigIntError, ToBigInt};
use xsd_macro_utils::UtilsDefaultSerde;

// https://www.w3.org/TR/xmlschema-2/#integer
#[derive(Default, Clone, PartialEq, PartialOrd, Debug, UtilsDefaultSerde)]
pub struct Integer(pub BigInt);

impl Integer {
    pub fn from_bigint(bigint: BigInt) -> Self {
        Integer(bigint)
    }
}

impl ToBigInt for Integer {
    fn to_bigint(&self) -> Option<BigInt> {
        Some(self.0.clone())
    }
}

impl FromStr for Integer {
    type Err = ParseBigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Integer(BigInt::from_str(s)?))
    }
}

impl fmt::Display for Integer {
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
    fn integer_parse_test() {
        assert_eq!(
            Integer::from_str("12678967543233"),
            Ok(Integer(BigInt::from_str("12678967543233").unwrap()))
        );

        assert_eq!(Integer::from_str("+100000"), Ok(Integer(100000.to_bigint().unwrap())));

        assert_eq!(Integer::from_str("0"), Ok(Integer(0.to_bigint().unwrap())));

        assert_eq!(Integer::from_str("-1"), Ok(Integer((-1).to_bigint().unwrap())));

        // Invalid values.
        assert!(Integer::from_str("A").is_err());
        assert!(Integer::from_str("--1").is_err());
        assert!(Integer::from_str("++1").is_err());
        assert!(Integer::from_str("-+1").is_err());
    }

    #[test]
    fn integer_display_test() {
        assert_eq!(
            Integer(BigInt::from_str("12678967543233").unwrap()).to_string(),
            "12678967543233"
        );

        assert_eq!(Integer(100000.to_bigint().unwrap()).to_string(), "100000");

        assert_eq!(Integer(0.to_bigint().unwrap()).to_string(), "0");

        assert_eq!(Integer((-1).to_bigint().unwrap()).to_string(), "-1");
    }

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespaces = {"t" = "test"})]
    pub struct IntegerPair {
        #[yaserde(prefix = "t", rename = "First")]
        pub first: Integer,

        #[yaserde(prefix = "t", rename = "Second")]
        pub second: Integer,
    }

    #[test]
    fn integer_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:IntegerPair xmlns:t="test">
                <t:First>1234</t:First>
                <t:Second>-1234</t:Second>
            </t:IntegerPair>
            "#;
        let i = IntegerPair {
            first: Integer::from_bigint(1234.to_bigint().unwrap()),
            second: Integer::from_bigint((-1234).to_bigint().unwrap()),
        };
        let actual = yaserde::ser::to_string(&i).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn integer_deserialize_test() {
        // Value "+1234" is used to check optional plus sign deserialization.
        let s = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:IntegerPair xmlns:t="test">
                <t:First>+1234</t:First>
                <t:Second>-1234</t:Second>
            </t:IntegerPair>
            "#;
        let i: IntegerPair = yaserde::de::from_str(s).unwrap();
        assert_eq!(i.first.to_bigint().unwrap(), 1234.to_bigint().unwrap());
        assert_eq!(i.second.to_bigint().unwrap(), (-1234).to_bigint().unwrap());
    }
}
