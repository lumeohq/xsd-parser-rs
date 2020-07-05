use crate::xml_to_xsd::schema_set::global_types_set::GlobalTypesSet;
use crate::xml_to_xsd::schema_set::schema_wrapper::SchemaWrapper;
use crate::xsd_model::simple_types::any_uri::AnyUri;
use crate::xsd_model::Schema;
use roxmltree::{Node, Document};
use std::collections::HashMap;
use crate::xml_to_xsd::{XsdNode, ElementChildren};
use crate::xsd_model::elements::ElementType;

mod global_types_set;
pub mod results;
pub mod schema_wrapper;

#[derive(Default)]
pub struct SchemaSet<'a> {
    schemas: Vec<(Schema<'a>, Node<'a, 'a>)>,
    types: HashMap<&'a str, GlobalTypesSet<'a>>,
}

impl<'a> SchemaSet<'a> {
    pub fn from_docs(docs: &'a [Document]) -> Result<Self, String> {
        let mut res = SchemaSet::default();
        for d in docs {
            let root = d.root_element();
            if root.xsd_type()? == ElementType::Schema {
                res.add_schema(root);
            } else {
                for ch in root.element_children() {
                    if ch.xsd_type()? == ElementType::Schema {
                        res.add_schema(root);
                    }
                }
            }
        }
        Ok(res)
    }

    pub fn add_schema(&mut self, node: Node<'a, '_>) -> Result<(), String> {
        let schema = Schema::parse(node)?;
        let ns = schema.target_namespace.as_ref().unwrap_or(&AnyUri(""));
        self.types
            .entry(ns.0)
            .or_insert_with(GlobalTypesSet::default)
            .add_schema(&schema)?;
        self.schemas.push((schema, node));
        Ok(())
    }

    pub fn schemas(&'a self) -> Vec<SchemaWrapper<'a>> {
        self.schemas
            .iter()
            .map(|sw| SchemaWrapper::new(self, &sw.0, sw.1))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::xml_to_xsd::schema_set::results::{AttributeBase, AttributeType};
    use crate::xml_to_xsd::schema_set::SchemaSet;
    use crate::xsd_model::groups::simple_derivation::SimpleDerivation;
    use crate::xsd_model::simple_types::qname::QName;
    use crate::xsd_model::simple_types::SimpleType;
    use roxmltree::Document;

    const TYPES: &str = include_str!("../../../../input/xsd/types.xsd");
    const RULES: &str = include_str!("../../../../input/xsd/rules.xsd");
    const COMMON: &str = include_str!("../../../../input/xsd/common.xsd");

    #[test]
    fn test_one_scheme() {
        let doc = Document::parse(TYPES).unwrap();
        let mut schema_set = SchemaSet::default();
        schema_set.add_schema(doc.root_element()).unwrap();

        let sn = schema_set.types.values().next().unwrap();
        assert_eq!(sn.custom_types.len(), 8);

        let schemas = schema_set.schemas();
        let schema_wrapper = schemas.first().unwrap();
        let st = schema_wrapper
            .resolve_attribute_type(&QName::new("Name"))
            .unwrap();

        if let AttributeType::Simple(v) = st {
            if let SimpleDerivation::Restriction(ref r) = v.content_choice {
                if let AttributeBase::Builtin(st) = schema_wrapper
                    .resolve_base(r.base.as_ref().unwrap())
                    .unwrap()
                {
                    assert_eq!(st, SimpleType::String);
                } else {
                    panic!("Test Failed!");
                }
            } else {
                panic!("Test Failed!");
            }
        } else {
            panic!("Test Failed!");
        }
    }

    #[test]
    fn test_many_schemes() {
        let mut schema_set = SchemaSet::default();

        let doc = Document::parse(RULES).unwrap();
        schema_set.add_schema(doc.root_element()).unwrap();

        let doc = Document::parse(COMMON).unwrap();
        schema_set.add_schema(doc.root_element()).unwrap();

        let schemas = schema_set.schemas();
        let rules_schema_wrapper = &schemas[0];
        let base = rules_schema_wrapper
            .resolve_base(&QName::new("tt:IntRange"))
            .unwrap();

        if let AttributeBase::Complex(v) = base {
            assert_eq!(v.name.0, "IntRange");
            assert_eq!(
                v.annotation.as_ref().unwrap().doc_str(0).unwrap(),
                "Range of values greater equal Min value and less equal Max value."
            );
        } else {
            panic!("Test Failed!");
        }
    }
}
