use crate::xsd_model::simple_types::SimpleType;
use crate::xsd_model::{TopLevelComplexType, TopLevelSimpleType};
use std::rc::Rc;

#[derive(Debug)]
pub enum AttributeType<'a> {
    Builtin(SimpleType),
    Simple(Rc<TopLevelSimpleType<'a>>),
}

#[derive(Debug)]
pub enum BuiltinOrCustom<'a> {
    Builtin(SimpleType),
    Simple(Rc<TopLevelSimpleType<'a>>),
    Complex(Rc<TopLevelComplexType<'a>>),
}

pub type ElementType<'a> = BuiltinOrCustom<'a>;
pub type AttributeBase<'a> = BuiltinOrCustom<'a>;
