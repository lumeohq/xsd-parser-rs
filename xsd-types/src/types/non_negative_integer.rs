use std::{fmt, str::FromStr};

use num_bigint::{BigUint, ToBigUint};
use xsd_macro_utils::UtilsDefaultSerde;

// https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger
#[derive(Default, Clone, PartialEq, PartialOrd, Debug, UtilsDefaultSerde)]
pub struct NonNegativeInteger(pub BigUint);

impl NonNegativeInteger {
    pub fn from_biguint(bigint: BigUint) -> Self {
        NonNegativeInteger(bigint)
    }
}

impl ToBigUint for NonNegativeInteger {
    fn to_biguint(&self) -> Option<BigUint> {
        Some(self.0.clone())
    }
}

impl FromStr for NonNegativeInteger {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = BigUint::from_str(s).map_err(|e| e.to_string())?;
        if value < 0.to_biguint().unwrap() {
            Err("Bad value for NonNegativeInteger".to_string())
        } else {
            Ok(NonNegativeInteger(value))
        }
    }
}

impl fmt::Display for NonNegativeInteger {
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
    fn non_negative_integer_parse_test() {
        assert_eq!(
            NonNegativeInteger::from_str("12678967543233"),
            Ok(NonNegativeInteger(BigUint::from_str("12678967543233").unwrap()))
        );

        assert_eq!(
            NonNegativeInteger::from_str("+100000"),
            Ok(NonNegativeInteger(100000.to_biguint().unwrap()))
        );

        assert_eq!(
            NonNegativeInteger::from_str("0"),
            Ok(NonNegativeInteger(0.to_biguint().unwrap()))
        );

        // Invalid values.
        assert!(NonNegativeInteger::from_str("-1").is_err());
        assert!(NonNegativeInteger::from_str("-1234").is_err());
        assert!(NonNegativeInteger::from_str("A").is_err());
        assert!(NonNegativeInteger::from_str("--1").is_err());
        assert!(NonNegativeInteger::from_str("++1").is_err());
        assert!(NonNegativeInteger::from_str("-+1").is_err());
    }

    #[test]
    fn non_negative_integer_display_test() {
        assert_eq!(
            NonNegativeInteger(BigUint::from_str("12678967543233").unwrap()).to_string(),
            "12678967543233"
        );

        assert_eq!(NonNegativeInteger(100000.to_biguint().unwrap()).to_string(), "100000");

        assert_eq!(NonNegativeInteger(0.to_biguint().unwrap()).to_string(), "0");
    }

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "t", namespaces = {"t" = "test"})]
    pub struct NonNegativeIntegerPair {
        #[yaserde(prefix = "t", rename = "First")]
        pub first: NonNegativeInteger,

        #[yaserde(prefix = "t", rename = "Second")]
        pub second: NonNegativeInteger,
    }

    #[test]
    fn non_negative_integer_serialize_test() {
        let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:NonNegativeIntegerPair xmlns:t="test">
                <t:First>1234</t:First>
                <t:Second>0</t:Second>
            </t:NonNegativeIntegerPair>
            "#;
        let i = NonNegativeIntegerPair {
            first: NonNegativeInteger::from_biguint(1234.to_biguint().unwrap()),
            second: NonNegativeInteger::from_biguint(0.to_biguint().unwrap()),
        };
        let actual = yaserde::ser::to_string(&i).unwrap();
        assert_xml_eq(&actual, expected);
    }

    #[test]
    fn non_negative_integer_deserialize_test() {
        // Value "+1234" is used to check optional plus sign deserialization.
        let s = r#"<?xml version="1.0" encoding="UTF-8"?>
            <t:NonNegativeIntegerPair xmlns:t="test">
                <t:First>+1234</t:First>
                <t:Second>0</t:Second>
            </t:NonNegativeIntegerPair>
            "#;
        let i: NonNegativeIntegerPair = yaserde::de::from_str(s).unwrap();
        assert_eq!(i.first.to_biguint().unwrap(), 1234.to_biguint().unwrap());
        assert_eq!(i.second.to_biguint().unwrap(), 0.to_biguint().unwrap());
    }
}
