use crate::generator::simple_type::*;
use crate::generator::utils::*;
use crate::xsd2::complex_type::{ComplexType};
use crate::xsd2::schema::Schema;
use crate::xsd2::simple_type::SimpleType;
use crate::generator::type_tree::{Types, TupleStruct, Struct};
use crate::generator::struct_field::{StructField, any_attribute_field, field_from_attribute, get_fields_from_sequence, get_fields_from_extension};
use crate::generator::enumeration::Enum;
use crate::generator::complex_type::{get_types_from_sequence, get_enum_from_choice};
use crate::xsd2::node_traits::Extension;

pub struct Generator<'a, 'input> {
    target_namespace: Option<&'a str>,
    pub schema: Schema<'a, 'input>,
    //types: Vec<Types>
}

impl <'a, 'input> Generator<'a, 'input> {
    pub fn new(schema: roxmltree::Node<'a, 'input>) -> Self {
        let sc = Schema::<'a, 'input>{node: schema};
        let tn = sc.target_namespace();
        Generator {
            target_namespace: tn,
            schema: sc,
            //types: vec!()
        }
    }

    pub fn print(&self) {
        for node in self.schema.node.
            children().
            filter(|node| node.is_element() ) {
                match node.tag_name().name() {
                        "simpleType" => println!("{}", self.simple_type(&SimpleType{node})),
                        "complexType" => for t in self.complex_type(&ComplexType{node}) {println!("{}", t);}
                    ,
                    _ =>  print!("")
                }
        }
    }

    fn complex_type(&self, element: &ComplexType) -> Vec<Types> {
        let mut types: Vec<Types> = vec!();
        let comment = get_structure_comment(element.documentation());
        let name = match_type(
            element.name().expect("GLOBAL COMPLEX TYPE NAME REQUIRED"),
            self.target_namespace
        ).to_string();

        let mut fields = element.
            attributes().
            iter().
            map(|a| field_from_attribute(a, self.target_namespace)).
            collect::<Vec<StructField>>();

        match element.sequence()  {
            Some(s) => {
                fields.append(&mut get_fields_from_sequence(&s, self.target_namespace));
                let mut seq_types = get_types_from_sequence(&s, &name, self.target_namespace);
                for ty in &seq_types {
                    match ty {
                        Types::Enum(en) => {
                            fields.push(StructField{
                                name: get_field_name(en.name.as_str()),
                                typename: match_type(en.name.as_str(), self.target_namespace).to_string(),
                                comment: String::new(),
                                macros: "//TODO: add yaserde macros\n".to_string()
                            }
                            )
                        },
                        _ => ()
                    }

                }
                types.append(&mut seq_types)
            },
            None => ()
        }

        match element.choice() {
            Some(ch) => {
                let en = get_enum_from_choice(&ch, &name, self.target_namespace);
                fields.push(StructField{
                                name: get_field_name(en.name.as_str()),
                                typename: match_type(en.name.as_str(), self.target_namespace).to_string(),
                                comment: String::new(),
                                macros: "//TODO: add yaserde macros\n".to_string()
                            }
                );
                types.push(Types::Enum(en));
            },
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

        types.push(Types::Struct(Struct{
            comment,
            name,
            macros: String::new(),
            fields,
        }));

        types
    }

    fn simple_type(&self, element: &SimpleType) -> Types {
        let comment = get_structure_comment(element.documentation());
        let name = match_type(
            element.name().expect("SIMPLE TYPE WITHOUT NAME NOT SUPPORTED"),
            self.target_namespace
        ).to_string();
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
                    cases: get_enum_cases(&facets, self.target_namespace)
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

