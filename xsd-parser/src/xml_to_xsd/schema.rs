use crate::xml_to_xsd::{ElementChildren_, XsdNode};
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::schema_top::SchemaTop;
use crate::xsd_model::simple_types::any_uri::AnyUri;
use crate::xsd_model::simple_types::block_set::BlockSet;
use crate::xsd_model::simple_types::form_choice::FormChoice;
use crate::xsd_model::simple_types::full_derivation_set::FullDerivationSet;
use crate::xsd_model::simple_types::id::Id;
use crate::xsd_model::simple_types::language::Language;
use crate::xsd_model::simple_types::token::Token;
use crate::xsd_model::Annotation;
use crate::xsd_model::Import;
use crate::xsd_model::Include;
use crate::xsd_model::Schema;
use roxmltree::Document;

pub fn parse_document<'a>(doc: &'a Document) -> Result<Schema<'a>, String> {
    let schema_node = doc.root_element();
    let mut schema = Schema::default();

    schema.namespaces = Vec::from(schema_node.namespaces());

    for attr in schema_node.attributes() {
        match attr.name() {
            "targetNamespace" => schema.target_namespace = Some(AnyUri(attr.value())),
            "version" => schema.version = Some(Token(attr.value())),
            "finalDefault" => schema.final_default = Some(FullDerivationSet::parse(attr.value())?),
            "blockDefault" => schema.block_default = Some(BlockSet::parse(attr.value())?),
            "attributeFormDefault" => {
                schema.attribute_form_default = FormChoice::parse(attr.value())?
            }
            "elementFormDefault" => schema.element_form_default = FormChoice::parse(attr.value())?,
            "id" => schema.id = Some(Id(attr.value())),
            "lang" => schema.lang = Some(Language(attr.value())),
            _ => schema.attributes.push(attr.clone()),
        };
    }

    for ch in schema_node.element_children() {
        match ch.xsd_type()? {
            ElementType::Include => schema.includes.push(Include::parse(ch)?),
            ElementType::Import => schema.imports.push(Import::parse(ch)?),
            ElementType::Redefine => unimplemented!("Not present in ONVIF"),
            ElementType::Annotation => {
                if let Some(val) = schema.content.last_mut() {
                    val.1 = Some(Annotation::parse(ch)?)
                } else {
                    schema.annotations.push(Annotation::parse(ch)?)
                }
            }
            x => schema.content.push((SchemaTop::parse(ch, x)?, None)),
        }
    }

    Ok(schema)
}

#[cfg(test)]
mod test {
    use super::*;
    use roxmltree::Document;

    const TEXT: &str = include_str!("../../../input/xsd/onvif.xsd");
    #[test]
    fn test_parse_document() {
        let doc = Document::parse(TEXT).unwrap();
        let schema = parse_document(&doc).unwrap();
        assert_eq!(
            schema.target_namespace.unwrap().0,
            "http://www.onvif.org/ver10/schema"
        );
        assert_eq!(schema.final_default.unwrap(), FullDerivationSet::All);
        assert_eq!(schema.block_default.unwrap(), BlockSet::All);
        assert_eq!(schema.id.unwrap().0, "ID");
        assert_eq!(schema.element_form_default, FormChoice::Qualified);
        assert_eq!(schema.lang.unwrap().0, "us");
        assert_eq!(schema.imports.len(), 2);
        assert_eq!(schema.includes.len(), 2);
    }
}
