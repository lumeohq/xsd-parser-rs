use crate::generator::simple_type::*;
use crate::generator::utils::*;
use crate::xsd2::complex_type::{ComplexType};
use crate::xsd2::schema::{Schema, TargetNamespace};
use crate::xsd2::simple_type::SimpleType;
use crate::generator::type_tree::{Types, TupleStruct, Struct};
use crate::generator::struct_field::{StructField, any_attribute_field, field_from_attribute, get_fields_from_sequence, get_fields_from_extension};
use crate::generator::enumeration::Enum;
use crate::generator::complex_type::{get_types_from_sequence, get_enum_from_choice};
use crate::xsd2::node_traits::Extension;

pub struct Generator<'a, 'input> {
    target_namespace: Option<TargetNamespace<'a>>,
    pub schema: Schema<'a, 'input>,
    //types: Vec<Types>
}

impl <'a, 'input> Generator<'a, 'input> {
    pub fn new(schema: roxmltree::Node<'a, 'input>) -> Self {
        let sc = Schema::<'a, 'input>{node: schema};
        Generator {
            target_namespace: sc.target_namespace(),
            schema: sc,
            //types: vec!()
        }
    }

    pub fn print(&self) {
        for node in self.schema.node.children().filter(|node| node.is_element() ) {
            match node.tag_name().name() {
                "simpleType" => println!("{}", self.simple_type(&SimpleType{node})),
                "complexType" => for t in self.complex_type(&ComplexType{node}) {
                    println!("{}", t);
                },
                _ =>  print!("")
            }
        }
    }

    fn complex_type(&self, element: &ComplexType) -> Vec<Types> {
        let mut types: Vec<Types> = vec!();
        let comment = get_structure_comment(element.documentation());
        let macros = self.struct_macro();
        let name = match_type(
            element.name().expect("GLOBAL COMPLEX TYPE NAME REQUIRED"),
            self.target_namespace.as_ref()
        ).to_string();

        let mut fields = element.
            attributes().
            iter().
            map(|a| field_from_attribute(a, self.target_namespace.as_ref())).
            collect::<Vec<StructField>>();

        match element.sequence()  {
            Some(s) => {
                fields.append(&mut get_fields_from_sequence(&s, self.target_namespace.as_ref()));
                let mut seq_types = get_types_from_sequence(&s, &name, self.target_namespace.as_ref());
                for ty in &seq_types {
                    match ty {
                        Types::Enum(en) => {
                            fields.push(StructField{
                                name: get_field_name(en.name.as_str()),
                                type_name: match_type(en.name.as_str(), self.target_namespace.as_ref()).to_string(),
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
                let en = get_enum_from_choice(&ch, &name, self.target_namespace.as_ref());
                fields.push(StructField{
                                name: get_field_name(en.name.as_str()),
                                type_name: match_type(en.name.as_str(), self.target_namespace.as_ref()).to_string(),
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
                Some(ext) => fields.append(&mut  get_fields_from_extension(&ext, self.target_namespace.as_ref())),
                None => ()
            },
            None => ()
        }

        match element.simple_content() {
            Some(cc) => match cc.extension() {
                Some(ext) => fields.append(&mut  get_fields_from_extension(&ext, self.target_namespace.as_ref())),
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
            macros,
            fields,
        }));

        types
    }

    fn struct_macro(&self) -> String {
        let derives = "#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]\n";
        match &self.target_namespace {
            Some(tn) => format!("{derives}#[yaserde(prefix = \"{prefix}\", namespace = \"{prefix}: {uri}\")]\n",
                                derives=derives,
                                prefix=tn.prefix,
                                uri=tn.uri).to_string(),
            None => format!("{derives}#[yaserde()]\n", derives=derives).to_string()
        }
    }

    fn simple_type(&self, element: &SimpleType) -> Types {
        let comment = get_structure_comment(element.documentation());
        let name = match_type(
            element.name().expect("SIMPLE TYPE WITHOUT NAME NOT SUPPORTED"),
            self.target_namespace.as_ref()
        ).to_string();
        let l = element.list();
        let mut type_name = String::new();
        let restriction = element.restriction();
        if restriction.is_some() {
            let r = restriction.unwrap();
            type_name = match_type(&r.base(), self.target_namespace.as_ref()).to_string();
            let facets = get_enum_facets(&r);

            if !facets.is_empty() {
                return Types::Enum(Enum{
                    type_name,
                    name,
                    comment,
                    cases: get_enum_cases(&facets, self.target_namespace.as_ref())
                });
            }
        }
        else if l.is_some() {
            type_name = format!("Vec<{}>", match_type(
                &l.unwrap().item_type().unwrap_or("NESTED SIMPLE TYPE NOT SUPPORTED"),
                 self.target_namespace.as_ref()
            ).as_ref());
        }
        return Types::TupleStruct(TupleStruct{
            comment,
            name,
            type_name,
            macros: String::new()
        });
    }
}
