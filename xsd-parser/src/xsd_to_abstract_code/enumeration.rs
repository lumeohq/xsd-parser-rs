use crate::abstract_code_model::enumeration::{Enumeration, EnumItem};
use crate::xsd_model::{Choice, LocalElement};
use crate::xsd_model::groups::nested_particle::NestedParticle;
use crate::abstract_code_model::comment::Comment;

impl<'a> Enumeration<'a> {
    pub fn from_choice(choice: &'a Choice) -> Self {
        Enumeration {
            name: None,
            variants: EnumItem::from_choice(choice),
            comment: Comment::from_opt_annotation(&choice.annotation),
            type_name: None,
            subtypes: vec![]
        }
    }
}

pub struct S;


impl<'a> EnumItem<'a> {
    pub fn from_choice(choice: &'a Choice) -> Vec<EnumItem<'a>> {

        vec![]
    }

    pub fn from_nested_particle(np: &'a NestedParticle) -> Result<EnumItem<'a>, String> {
        match np {
            NestedParticle::Element(elem) => {},
            _ => {}
        }

        Err(String::default())
    }
}

// fn element_to_variant(elem: &Box<LocalElement>) -> Result<Variant, String> {
//     if let Some(ref_) = elem.ref_ {
//         Ok(
//             Variant {
//                 name: Some(ref_.name.0),
//                 comment: Default::default(),
//                 value: "".to_string(),
//                 type_name: None
//             }
//         )
//     } else if let Some(name) = elem.name {
//
//     } else {
//         Err("Either 'name' or 'ref' attribute should present".to_string())
//     }
// }