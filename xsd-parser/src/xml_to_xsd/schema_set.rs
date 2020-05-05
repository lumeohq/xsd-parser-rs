use crate::xsd_model::groups::schema_top::SchemaTop;
use crate::xsd_model::simple_types::any_uri::AnyUri;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::{SimpleType, xsd_simple_type};
use crate::xsd_model::{
    Schema, TopLevelAttribute, TopLevelComplexType, TopLevelElement, TopLevelSimpleType,
};
use roxmltree::Node;
use std::collections::HashMap;
use std::rc::Rc;
use crate::xml_to_xsd::XSD_NS_URI;

#[derive(Debug)]
pub enum CustomType<'a> {
    Simple(Rc<TopLevelSimpleType<'a>>),
    Complex(Rc<TopLevelComplexType<'a>>),
}

#[derive(Default, Debug)]
pub struct GlobalTypesSet<'a> {
    // Todo: think about better name
    pub attributes: HashMap<&'a str, Rc<TopLevelAttribute<'a>>>,
    pub elements: HashMap<&'a str, Rc<TopLevelElement<'a>>>,
    pub custom_types: HashMap<&'a str, CustomType<'a>>,
}

impl<'a> GlobalTypesSet<'a> {
    pub fn from_schema(schema: &Schema<'a>) -> Result<Self, String> {
        let mut ret_val = Self::default();
        ret_val.add_schema(schema);
        Ok(ret_val)
    }

    pub fn add_schema(&mut self, schema: &Schema<'a>) -> Result<(), String> {
        for (cont, _) in schema.content.as_slice() {
            match cont {
                SchemaTop::Attribute(val) => {
                    if let Some(_) = self.attributes.insert(val.name.0, val.clone()) {
                        return Err(format!("An attribute with name '{}' has already been declared in this namespace", val.name.0));
                    }
                }
                SchemaTop::Element(val) => {
                    if let Some(_) = self.elements.insert(val.name.0, val.clone()) {
                        return Err(format!(
                            "An element with name '{}' has already been declared in this namespace",
                            val.name.0
                        ));
                    }
                }
                SchemaTop::SimpleType(val) => {
                    if let Some(_) = self
                        .custom_types
                        .insert(val.name.0, CustomType::Simple(val.clone()))
                    {
                        return Err(format!("An attribute with name '{}' has already been declared in this namespace", val.name.0));
                    }
                }
                SchemaTop::ComplexType(val) => {
                    if let Some(_) = self
                        .custom_types
                        .insert(val.name.0, CustomType::Complex(val.clone()))
                    {
                        return Err(format!("An attribute with name '{}' has already been declared in this namespace", val.name.0));
                    }
                }
                _ => {} //TODO: Group, AttributeGroup and Notation
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct SchemaSet<'a> {
    schemas: Vec<(Schema<'a>, Node<'a, 'a>)>,
    types: HashMap<&'a str, GlobalTypesSet<'a>>,
}

impl<'a> SchemaSet<'a> {
    pub fn add_schema(&mut self, node: Node<'a, '_>) -> Result<(), String> {
        let schema = Schema::parse(node.clone()).unwrap();
        let ns = schema.target_namespace.as_ref().unwrap_or(&AnyUri(""));
        self.types
            .entry(ns.0)
            .or_insert(GlobalTypesSet::default())
            .add_schema(&schema)?;
        self.schemas.push((schema, node));
        Ok(())
    }

    pub fn schemas(&'a self) -> Vec<SchemaWrapper<'a>> {
        self.schemas
            .iter()
            .map(|sw| SchemaWrapper {
                schema_set: self,
                schema: &sw.0,
                node: sw.1.clone(),
            })
            .collect()
    }
}

pub enum GlobalType<'a> {
    Builtin(SimpleType),
    Custom(SchemaTop<'a>),
}

pub struct SchemaWrapper<'a> {
    schema_set: &'a SchemaSet<'a>,
    schema: &'a Schema<'a>,
    node: Node<'a, 'a>,
}

pub enum AttributeType<'a> {
    Builtin(SimpleType),
    Simple(Rc<TopLevelSimpleType<'a>>)
}

pub enum AttributeBase<'a> {
    Builtin(SimpleType),
    Simple(Rc<TopLevelSimpleType<'a>>),
    Complex(Rc<TopLevelComplexType<'a>>),
}

impl<'a> SchemaWrapper<'a> {
    pub fn schema(&self) -> &Schema<'a> {
        &self.schema
    }

    pub fn resolve_attribute_ref(&self, name: &QName) -> Option<&TopLevelAttribute<'a>> {
        let ns_uri = self.node.lookup_namespace_uri(name.prefix);
        self.schema_set
            .types
            .get(ns_uri.unwrap_or(""))
            .and_then(|gts| gts.attributes.get(name.name).map(|v| v.as_ref()))
    }

    pub fn resolve_attribute_type(&self, name: &QName) -> Result<AttributeType, String> {
        let ns_uri = self.node.lookup_namespace_uri(name.prefix);
        if let Some(uri) = ns_uri {
            if uri == XSD_NS_URI {
                return Ok(AttributeType::Builtin(xsd_simple_type(name.name)?));
            }
        }
        self.schema_set
            .types
            .get(ns_uri.unwrap_or(""))
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

    pub fn resolve_attribute_base(&self, name: &QName) -> Result<AttributeBase, String> {
        let ns_uri = self.node.lookup_namespace_uri(name.prefix);
        if let Some(uri) = ns_uri {
            if uri == XSD_NS_URI {
                return Ok(AttributeBase::Builtin(xsd_simple_type(name.name)?));
            }
        }
        self.schema_set
            .types
            .get(ns_uri.unwrap_or(""))
            .and_then(|gts|
                gts.custom_types.get(name.name).map(|v| match v {
                    CustomType::Simple(val) => AttributeBase::Simple(val.clone()),
                    CustomType::Complex(val) => AttributeBase::Complex(val.clone()),
                })).ok_or_else(|| format!("Type {} not declared", name))
    }

}

#[cfg(test)]
mod test {
    use crate::xml_to_xsd::schema_set::{GlobalTypesSet, SchemaSet};
    use crate::xsd_model::Schema;
    use roxmltree::Document;

    #[test]
    fn test() {
        const TEXT: &str = include_str!("../../../input/xsd/types.xsd");
        let doc = Document::parse(TEXT).unwrap();
        let mut schema_set = SchemaSet::default();
        schema_set.add_schema(doc.root_element()).unwrap();

        let sn = schema_set.types.values().next().unwrap();

        println!("{}", sn.custom_types.len());
        println!("{}", sn.attributes.len());
        println!("{}", sn.elements.len());
    }
}
