use crate::xml_to_xsd::schema_set::global_types_set::CustomType;
use crate::xml_to_xsd::schema_set::results::{AttributeBase, AttributeType};
use crate::xml_to_xsd::schema_set::SchemaSet;
use crate::xml_to_xsd::XSD_NS_URI;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::xsd_simple_type;
use crate::xsd_model::{Schema, TopLevelAttribute, TopLevelElement};
use roxmltree::Node;

pub struct SchemaWrapper<'a> {
    schema_set: &'a SchemaSet<'a>,
    schema: &'a Schema<'a>,
    node: Node<'a, 'a>,
}

impl<'a> SchemaWrapper<'a> {
    pub fn new(schema_set: &'a SchemaSet<'a>, schema: &'a Schema<'a>, node: Node<'a, 'a>) -> Self {
        Self {
            schema_set,
            schema,
            node,
        }
    }

    pub fn schema(&self) -> &Schema<'a> {
        &self.schema
    }

    // for 'ref' attribute in xs:element
    pub fn find_element(&self, name: &QName) -> Option<&TopLevelElement<'a>> {
        let ns_uri = self.get_ns_uri(name);
        self.schema_set
            .types
            .get(ns_uri)
            .and_then(|gts| gts.elements.get(name.name).map(|v| v.as_ref()))
    }

    // for 'ref' attribute in xs:attribute
    pub fn find_attribute(&self, name: &QName) -> Option<&TopLevelAttribute<'a>> {
        let ns_uri = self.get_ns_uri(name);
        self.schema_set
            .types
            .get(ns_uri)
            .and_then(|gts| gts.attributes.get(name.name).map(|v| v.as_ref()))
    }

    // for 'type' attribute for xs:attribute
    pub fn resolve_attribute_type(&self, name: &QName) -> Result<AttributeType, String> {
        let ns_uri = self.get_ns_uri(name);
        if ns_uri == XSD_NS_URI {
            return Ok(AttributeType::Builtin(xsd_simple_type(name.name)?));
        }
        self.schema_set
            .types
            .get(ns_uri)
            .ok_or_else(|| format!("Unknown namespace in qname: {}", name))
            .and_then(|gts|
                gts
                    .custom_types
                    .get(name.name)
                    .ok_or_else(|| format!("Can't find simpleType with name: {}", name))
                    .and_then(|v| if let CustomType::Simple(val) = v {
                            Ok(AttributeType::Simple(val.clone()))
                        } else {
                            Err(format!("Attribute type must specify a built-in data type or a simple type: {}", name))
                        }
                    )
            )
    }

    // for 'base' attribute in xs:restriction and xs:extension
    pub fn resolve_base(&self, name: &QName) -> Result<AttributeBase, String> {
        let ns_uri = self.get_ns_uri(name);
        if ns_uri == XSD_NS_URI {
            return Ok(AttributeBase::Builtin(xsd_simple_type(name.name)?));
        }
        self.schema_set
            .types
            .get(ns_uri)
            .and_then(|gts| {
                gts.custom_types.get(name.name).map(|v| match v {
                    CustomType::Simple(val) => AttributeBase::Simple(val.clone()),
                    CustomType::Complex(val) => AttributeBase::Complex(val.clone()),
                })
            })
            .ok_or_else(|| format!("Type {} not declared", name))
    }

    fn get_ns_uri(&self, name: &'a QName) -> &str {
        if name.prefix.is_some() {
            self.node.lookup_namespace_uri(name.prefix)
        } else {
            self.schema.target_namespace.as_ref().map(|val| val.0)
        }
        .unwrap_or("")
    }
}
