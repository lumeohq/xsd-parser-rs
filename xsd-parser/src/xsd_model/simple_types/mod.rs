pub mod any_uri;
pub mod base64binary;
pub mod block_set;
pub mod boolean;
pub mod byte;
pub mod date;
pub mod datetime;
pub mod decimal;
pub mod derivation_set;
pub mod deriviation_control;
pub mod duration;
pub mod form_choice;
pub mod full_derivation_set;
pub mod gday;
pub mod gmonth;
pub mod gmonthday;
pub mod gyear;
pub mod gyearmonth;
pub mod id;
pub mod integer;
pub mod language;
pub mod ncname;
pub mod negative_integer;
pub mod non_negative_integer;
pub mod non_positive_integer;
pub mod positive_integer;
pub mod public;
pub mod qname;
pub mod simple_derivation_set;
pub mod string;
pub mod time;
pub mod token;

pub type AnySimpleType<'a> = &'a str;
pub type Id<'a> = Option<id::Id<'a>>;

pub enum SimpleType {
    AnyURI,
    Base64Binary,
    Boolean,
    Byte,
    Date,
    DateTime,
    Decimal,
    DerivationControl,
    Double,
    Duration,
    ENTITIES,
    ENTITY,
    Float,
    GDay,
    GMonth,
    GMonthDay,
    GYear,
    GYearMonth,
    HexBinary,
    ID,
    IDREF,
    IDREFS,
    Int,
    Integer,
    Language,
    Long,
    Name,
    NCName,
    NegativeInteger,
    NMTOKEN,
    NMTOKENS,
    NonNegativeInteger,
    NonPositiveInteger,
    NormalizedString,
    NOTATION,
    PositiveInteger,
    QName,
    Short,
    SimpleDerivationSet,
    String_,
    Time,
    Token,
    UnsignedByte,
    UnsignedInt,
    UnsignedLong,
    UnsignedShort,
}

pub fn xsd_simple_type(name: &str) -> Result<SimpleType, String> {
    use SimpleType::*;
    Ok(match name {
        "anyURI" => AnyURI,
        "base64Binary" => Base64Binary,
        "boolean" => Boolean,
        "byte" => Byte,
        "date" => Date,
        "dateTime" => DateTime,
        "decimal" => Decimal,
        "derivationControl" => DerivationControl,
        "double" => Double,
        "duration" => Duration,
        "ENTITIES" => ENTITIES,
        "ENTITY" => ENTITY,
        "float" => Float,
        "gDay" => GDay,
        "gMonth" => GMonth,
        "gMonthDay" => GMonthDay,
        "gYear" => GYear,
        "gYearMonth" => GYearMonth,
        "hexBinary" => HexBinary,
        "ID" => ID,
        "IDREF" => IDREF,
        "IDREFS" => IDREFS,
        "int" => Int,
        "integer" => Integer,
        "language" => Language,
        "long" => Long,
        "name" => Name,
        "NCName" => NCName,
        "negativeInteger" => NegativeInteger,
        "NMTOKEN" => NMTOKEN,
        "NMTOKENS" => NMTOKENS,
        "nonNegativeInteger" => NonNegativeInteger,
        "nonPositiveInteger" => NonPositiveInteger,
        "normalizedString" => NormalizedString,
        "NOTATION" => NOTATION,
        "positiveInteger" => PositiveInteger,
        "QName" => QName,
        "short" => Short,
        "simpleDerivationSet" => SimpleDerivationSet,
        "string" => String_,
        "time" => Time,
        "token" => Token,
        "unsignedByte" => UnsignedByte,
        "unsignedInt" => UnsignedInt,
        "unsignedLong" => UnsignedLong,
        "unsignedShort" => UnsignedShort,
        _ => return Err(format!("Invalid xs:simpleType name: {}", name)),
    })
}
