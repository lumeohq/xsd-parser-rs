use crate::xsd_model::groups::schema_top::SchemaTop;
use crate::xsd_model::{
    Schema, TopLevelAttribute, TopLevelComplexType, TopLevelElement, TopLevelSimpleType,
};
use std::collections::HashMap;
use std::rc::Rc;

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
        ret_val.add_schema(schema)?;
        Ok(ret_val)
    }

    pub fn add_schema(&mut self, schema: &Schema<'a>) -> Result<(), String> {
        for (cont, _) in schema.content.as_slice() {
            match cont {
                SchemaTop::Attribute(val) => {
                    if self.attributes.insert(val.name.0, val.clone()).is_some() {
                        return Err(format!("An attribute with name '{}' has already been declared in this namespace", val.name.0));
                    }
                }
                SchemaTop::Element(val) => {
                    if self.elements.insert(val.name.0, val.clone()).is_some() {
                        return Err(format!(
                            "An element with name '{}' has already been declared in this namespace",
                            val.name.0
                        ));
                    }
                }
                SchemaTop::SimpleType(val) => {
                    if self
                        .custom_types
                        .insert(val.name.0, CustomType::Simple(val.clone()))
                        .is_some()
                    {
                        return Err(format!("An attribute with name '{}' has already been declared in this namespace", val.name.0));
                    }
                }
                SchemaTop::ComplexType(val) => {
                    if self
                        .custom_types
                        .insert(val.name.0, CustomType::Complex(val.clone()))
                        .is_some()
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
