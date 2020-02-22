# xsd-parser-rs
An xsd/wsdl => rust code generator written in rust. The main target is for generation of the [ONVIF Specifications](https://www.onvif.org/) but should work for other SOAP/XSDL/WSDL needs.

## Work in Progress
This is still a work in progress.  So please feel free to open issues and submit PRs. Please be sure to read and follow our [Code of Conduct](/CODE_OF_CONDUCT.md).

## XSD types mapping

A following mapping used to represent built-in XSD types as rust types:

|XSD               |rust         |
|------------------|-------------|
|hexBinary         |String       | 
|base64Binary      |String       |
|boolean           |bool         |
|integer           |i64 (1)      |
|nonNegativeInteger|i64 (1)      |
|positiveInteger   |i64 (1)      |
|nonPositiveInteger|i64 (1)      |
|negativeInteger   |i64 (1)      |
|long              |i64          |
|int               |i32          |
|short             |i16          |
|byte              |i8           |
|unsignedLong      |u64          |
|unsignedInt       |u32          |
|unsignedShort     |u16          |
|unsignedByte      |u8           |
|decimal           |f64 (2)      |
|double            |f64          |
|float             |f64          |
|date              |String (3)   |
|time              |String (3)   |
|dateTime          |String (3)   |
|dateTimeStamp     |String (3)   |
|duration          |Duration (4) |
|gDay              |String (5)   |
|gMonth            |String (5)   |
|gMonthDay         |String (5)   |
|gYear             |String (5)   |
|gYearMonth        |String (5)   |
|string            |String       |
|normalizedString  |String       |
|token             |String       |
|language          |String       |
|Name              |String       |
|NCName            |String       |
|ENTITY            |String       |
|ID                |String       |
|IDREF             |String       |
|NMTOKEN           |String       |
|anyURI            |String       |
|QName             |String       |
|NOTATION          |String       |
|ENTITIES          |Vec\<String\>|
|IDREFS            |Vec\<String\>|
|NMTOKENS          |Vec\<String\>|

Notes:

(1) we are going to use num_bigint::Bigintg type or its analog in the future

(2) we are going to use bigdecimal::BigDecimal type or its analog in the future

(3) we are going to use types from the chrono crate or its analog in the future

(4) we are using our own type for duration, since there are no known implementation
in rust that supports proper month/years holding and literal representation

(5) we are going to implement types that both provide stored value as integer and
support proper (de)serialization

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
