use std::borrow::Cow;

use inflector::cases::snakecase::to_snake_case;

use crate::generator::complex_type::{field_from_attribute, yaserde_attributes};
use crate::generator::enumeration::enum_struct;
use crate::generator::simple_type::*;
use crate::generator::utils::*;
use crate::xsd2::complex_type::{Attribute, ComplexType};
use crate::xsd2::schema::Schema;
use crate::xsd2::simple_type::SimpleType;

pub struct Generator<'a, 'input> {
    target_namespace: Option<&'a str>,
    pub schema: Schema<'a, 'input>,
}

impl <'a, 'input> Generator<'a, 'input> {
    pub fn new(schema: roxmltree::Node<'a, 'input>) -> Self {
        let sc = Schema::<'a, 'input>{node: schema};
        let tn = sc.target_namespace();
        Generator {
            target_namespace: tn,
            schema: sc,
        }
    }

    pub fn print(&self) {
        for st in self.schema.node.
        children().
        filter(|node| node.is_element() && node.tag_name().name() == "simpleType").
        map(|node| SimpleType{node}).collect::<Vec<SimpleType>>() {
            println!("{}", self.simple_type(&st));
        }

        for node in self.schema.node.
            children().
            filter(|node| node.is_element() ) {
            match node.tag_name().name() {
                "simpleType" => println!("{}", self.simple_type(&SimpleType{node})),
                "complexType" => println!("{}", self.complex_type(&ComplexType{node})),
                _ =>  println!("{:?}\n\n", node)
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

    fn complex_type(&self, element: &ComplexType) -> String {
        let doc = get_comment(element.documentation());
        let name = get_type_name(element.name().expect("GLOBAL COMPLEX TYPE NAME REQUIRED"));
        let attributes = element.
            attributes().
            iter().
            map(|a| self.field_from_attribute(a)).
            collect::<Vec<String>>().
            join("\n\n");
        format!("{}{}pub struct {} {{\n{}\n}} \n\n", doc, yaserde_derive(), name, attributes)
    }

    fn field_from_attribute(&self, attr: &Attribute) -> String {
        let name = attr.name();
        format!("  {}\n  pub {}: {},  {}",
                yaserde_attributes(name),
                to_snake_case(&name),
                self.match_type(attr.typename()),
                get_comment(attr.documentation())
        )
    }

    fn simple_type(&self, element: &SimpleType) -> String {

        let doc = get_comment(element.documentation());
        let name = get_type_name(element.name().expect("SIMPLE TYPE WITHOUT NAME NOT SUPPORTED"));
        let l = element.list();
        if l.is_some() {
            return list_simple_type(
                &doc,
                &name,
                self.match_type(&l.unwrap().item_type().unwrap_or("NESTED SIMPLE TYPE NOT SUPPORTED")).as_ref());
        }

        let restriction = element.restriction();
        if restriction.is_some() {
            let r = restriction.unwrap();
            let typename = self.match_type(r.base());
            let facets = get_enum_facets(&r);

            if !facets.is_empty() {
                return enum_struct(
                    &name,
                    &facets,
                    typename
                );
            }

            return tuple_struct(&doc, &name, typename);
        }

        return format!("{} = {}", "UNSUPPORTED SIMPLE TYPE", element);
    }
}
