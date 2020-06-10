extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

mod tuple;
mod union;

#[proc_macro_derive(UtilsTupleIo)]
pub fn tuple_serde(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let from_str = tuple::from_str(&ast);
    let display = tuple::display(&ast);

    let ts = quote! {
        #from_str
        #display
    };

    ts.into()
}

// Adds YaSerialize and YaDeserialize implementations for types that support FromStr and Display traits.
#[proc_macro_derive(UtilsDefaultSerde)]
pub fn default_serde(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident;
    let struct_name_literal = &ast.ident.to_string();

    let serde = quote! {
        impl YaSerialize for #struct_name {
            fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
                utils::yaserde::serialize(self, #struct_name_literal, writer, |s| s.to_string())
            }

            fn serialize_attributes(&self, attributes: Vec<xml::attribute::OwnedAttribute>, namespace: xml::namespace::Namespace) -> Result<(Vec<xml::attribute::OwnedAttribute>, xml::namespace::Namespace), String> {
                Ok((attributes, namespace))
            }
        }

        impl YaDeserialize for #struct_name {
            fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
                utils::yaserde::deserialize(reader, |s| #struct_name::from_str(s).map_err(|e| e.to_string()))
            }
        }
    };

    serde.into()
}

#[proc_macro_derive(UtilsUnionSerDe)]
pub fn union_serde(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let serde = union::serde(&ast);

    let ts = quote! {
        #serde
    };

    ts.into()
}
