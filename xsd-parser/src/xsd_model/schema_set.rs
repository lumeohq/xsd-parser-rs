use crate::xsd_model::Schema;

pub struct SchemaSet<'a> {
    schams: Vec<SchemaWrapper<'a>>,
}

struct SchemaWrapper<'a> {
    schema: Schema<'a>,
}

//QName -> TopLevelElement
