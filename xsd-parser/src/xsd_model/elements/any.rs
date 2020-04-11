use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::{RawAttribute, MaxOccurs};
use crate::xsd_model::simple_types::non_negative_integer::NonNegativeInteger;
use crate::xsd_model::simple_types::Id;

pub struct Any<'a> {
    annotation: Option<Annotation<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    namespace: &'a str, //TODO: namespaceList
    process_contents: &'a str,
    min_occurs: NonNegativeInteger,
    max_occurs: MaxOccurs,
}