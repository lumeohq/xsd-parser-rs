use crate::abstract_code_model::id::Id;
use crate::xsd_model::simple_types::Id as OptionalXsdId;

impl<'a> Id<'a> {
    pub fn parse(id: &OptionalXsdId<'a>) -> Id<'a> {
        Id(id.as_ref().map(|x| x.0))
    }
}

#[cfg(test)]
mod test {
    use crate::abstract_code_model::id::Id;
    use crate::xsd_model::simple_types::id::Id as XsdId;
    use crate::xsd_model::simple_types::Id as OptionalXsdId;

    #[test]
    fn test_id_parse() {
        let id: OptionalXsdId = Some(XsdId("An id"));
        let res = Id::parse(&id);
        assert_eq!(res.0, Some("An id"));
    }
}
