use std::borrow::{Cow};

use crate::generator::complex_type::{yaserde_for_attribute, attribute_type, element_type, yaserde_for_element};
use crate::generator::simple_type::*;
use crate::generator::utils::*;
use crate::xsd2::complex_type::{Attribute, ComplexType};
use crate::xsd2::schema::Schema;
use crate::xsd2::simple_type::SimpleType;
use crate::xsd2::sequence::Element;
use crate::generator::type_tree::{Types, TupleStruct, Enum, Struct, StructField};
use crate::xsd2::complex_content::ComplexContent;

pub struct Generator<'a, 'input> {
    target_namespace: Option<&'a str>,
    pub schema: Schema<'a, 'input>,
    types: Vec<Types>
}

impl <'a, 'input> Generator<'a, 'input> {
    pub fn new(schema: roxmltree::Node<'a, 'input>) -> Self {
        let sc = Schema::<'a, 'input>{node: schema};
        let tn = sc.target_namespace();
        Generator {
            target_namespace: tn,
            schema: sc,
            types: vec!()
        }
    }

    pub fn print(&self) {
        for node in self.schema.node.
            children().
            filter(|node| node.is_element() ) {
            match node.tag_name().name() {
                "simpleType" => println!("{}", self.simple_type(&SimpleType{node})),
                "complexType" => println!("{}", self.complex_type(&ComplexType{node})),
                _ =>  print!("")
            }
        }
    }

    fn match_type(&self, typename: &str) -> Cow<'a, str>{
        match typename {
            "xs:string"      => Cow::Borrowed("String"),
            "xs:NCName"      => Cow::Borrowed("String"),
            "xs:unsignedInt" => Cow::Borrowed("usize"),
            "xs:int"         => Cow::Borrowed("i64"),
            "xs:float"       => Cow::Borrowed("f64"),
            "xs:boolean"     => Cow::Borrowed("bool"),
            x => Cow::Owned(
                    match self.target_namespace {
                        Some(ns) => {
                            if x.starts_with(ns) { x[ns.len()+1..].to_string() }
                            else { x.replace(":", "::") }
                        },
                        None => x.replace(":", "::")
                    }
                )
        }
    }

    fn field_from_attribute(&self, attr: &Attribute) -> StructField {
        let name = attr.name();
        StructField{
            name: get_type_name(&name),
            typename: attribute_type(attr, self.match_type(attr.typename())),
            macros: yaserde_for_attribute(name),
            comment: get_field_comment(attr.documentation())
        }
    }

    fn field_from_element(&self, elem: &Element) -> StructField {
        let name = elem.name();
        StructField{
            name: get_type_name(&name),
            typename: element_type(elem, self.match_type(elem.typename())),
            macros: yaserde_for_element(name),
            comment: get_field_comment(elem.documentation())
        }
    }

    fn complex_type(&self, element: &ComplexType) -> Types {
        let comment = get_structure_comment(element.documentation());
        let name = get_type_name(element.name().expect("GLOBAL COMPLEX TYPE NAME REQUIRED"));

        let mut attributes = element.
            attributes().
            iter().
            map(|a| self.field_from_attribute(a)).
            collect::<Vec<StructField>>();

        let sequence = element.complex_content().
            and_then(|cc| cc.extension()).
            and_then(|ext| ext.sequence());


        let mut elements = match sequence {
            Some(s) =>
                s.elements().
                    iter().
                    map(|el| self.field_from_element(el)).
                    collect::<Vec<StructField>>(),
            None => match element.sequence()  {
                Some(s) => s.elements().
                                    iter().
                                    map(|el| self.field_from_element(el)).
                                    collect::<Vec<StructField>>(),
                None => vec!()
            }
        };
        let mut fields: Vec<StructField> = Vec::with_capacity(attributes.len() + elements.len());
        fields.append(&mut elements);
        fields.append(&mut attributes);

        match element.complex_content().and_then(|cc| cc.extension()) {
            Some(ext) => {
                let ty = ext.base();
                fields.push(StructField{
                    name: "base".to_string(),
                    typename: get_type_name(self.match_type(ty).as_ref()),
                    macros: yaserde_for_element("base"), //TODO: yaserde for base element
                    comment: String::new()
                });
            },
            None => ()
        }

        Types::Struct(Struct{
            comment,
            name,
            macros: String::new(),
            fields,
        })
    }

    fn simple_type(&self, element: &SimpleType) -> Types {

        let comment = get_structure_comment(element.documentation());
        let name = get_type_name(element.name().expect("SIMPLE TYPE WITHOUT NAME NOT SUPPORTED"));
        let l = element.list();
        let mut typename = String::new();
        let restriction = element.restriction();
        if restriction.is_some() {
            let r = restriction.unwrap();
            typename = self.match_type(&r.base()).to_string();
            let facets = get_enum_facets(&r);

            if !facets.is_empty() {
                return Types::Enum(Enum{
                    typename,
                    name,
                    comment,
                    cases: enum_cases2(&facets)
                });
            }
        }
        else if l.is_some() {
            typename = format!("Vec<{}>", self.match_type(&l.unwrap().item_type().unwrap_or("NESTED SIMPLE TYPE NOT SUPPORTED")).as_ref());
        }
        return Types::TupleStruct(TupleStruct{
            comment,
            name,
            typename,
            macros: String::new()
        });
    }

}

