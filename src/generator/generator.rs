use crate::generator::complex_type::{yaserde_for_element};
use crate::generator::simple_type::*;
use crate::generator::utils::*;
use crate::xsd2::complex_type::{ComplexType};
use crate::xsd2::schema::Schema;
use crate::xsd2::simple_type::SimpleType;
use crate::generator::type_tree::{Types, TupleStruct, Enum, Struct};
use crate::generator::struct_field::{StructField, any_attribute_field, field_from_attribute, get_fields_from_sequence, get_fields_from_extension};

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

    fn complex_type(&self, element: &ComplexType) -> Types {
        let comment = get_structure_comment(element.documentation());
        let name = get_type_name(
            element.name().expect("GLOBAL COMPLEX TYPE NAME REQUIRED"),
            self.target_namespace
        );

        let mut fields = element.
            attributes().
            iter().
            map(|a| field_from_attribute(a, self.target_namespace)).
            collect::<Vec<StructField>>();

        match element.sequence()  {
            Some(s) => fields.append(&mut get_fields_from_sequence(&s, self.target_namespace)),
            None => ()
        }

        match element.complex_content() {
            Some(cc) => match cc.extension() {
                Some(ext) => fields.append(&mut  get_fields_from_extension(&ext, self.target_namespace)),
                None => ()
            },
            None => ()
        }

        match element.simple_content() {
            Some(cc) => match cc.extension() {
                Some(ext) => fields.append(&mut  get_fields_from_extension(&ext, self.target_namespace)),
                None => ()
            },
            None => ()
        }

        if element.has_any_attribute() {
            fields.push(any_attribute_field());
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
        let name = get_type_name(
            element.name().expect("SIMPLE TYPE WITHOUT NAME NOT SUPPORTED"),
            self.target_namespace
        );
        let l = element.list();
        let mut typename = String::new();
        let restriction = element.restriction();
        if restriction.is_some() {
            let r = restriction.unwrap();
            typename = match_type(&r.base(), self.target_namespace).to_string();
            let facets = get_enum_facets(&r);

            if !facets.is_empty() {
                return Types::Enum(Enum{
                    typename,
                    name,
                    comment,
                    cases: enum_cases2(&facets, self.target_namespace)
                });
            }
        }
        else if l.is_some() {
            typename = format!("Vec<{}>", match_type(
                &l.unwrap().item_type().unwrap_or("NESTED SIMPLE TYPE NOT SUPPORTED"),
                 self.target_namespace
            ).as_ref());
        }
        return Types::TupleStruct(TupleStruct{
            comment,
            name,
            typename,
            macros: String::new()
        });
    }

}

