use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod tuple;
mod union;

#[proc_macro_derive(UtilsTupleIo)]
pub fn tuple_serde(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    tuple::serde(&ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

// Adds YaSerialize and YaDeserialize implementations for types that support FromStr and Display traits.
#[proc_macro_derive(UtilsDefaultSerde)]
pub fn default_serde(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let struct_name_literal = &ast.ident.to_string();

    let serde = quote! {
        impl ::yaserde::YaSerialize for #struct_name {
            fn serialize<W: ::std::io::Write>(
                &self,
                writer: &mut ::yaserde::ser::Serializer<W>,
            ) -> ::std::result::Result<(), ::std::string::String> {
                ::xsd_types::utils::yaserde::serialize(
                    self,
                    #struct_name_literal,
                    writer, |s| s.to_string(),
                )
            }

            fn serialize_attributes(
                &self,
                attributes: ::std::vec::Vec<::xml::attribute::OwnedAttribute>,
                namespace: ::xml::namespace::Namespace,
            ) -> ::std::result::Result<
                (
                    ::std::vec::Vec<::xml::attribute::OwnedAttribute>,
                    ::xml::namespace::Namespace,
                ),
                ::std::string::String,
            > {
                Ok((attributes, namespace))
            }
        }

        impl ::yaserde::YaDeserialize for #struct_name {
            fn deserialize<R: ::std::io::Read>(
                reader: &mut ::yaserde::de::Deserializer<R>,
            ) -> ::std::result::Result<Self, ::std::string::String> {
                ::xsd_types::utils::yaserde::deserialize(
                    reader,
                    |s| #struct_name::from_str(s).map_err(|e| e.to_string()),
                )
            }
        }
    };

    serde.into()
}

#[proc_macro_derive(UtilsUnionSerDe)]
pub fn union_serde(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    union::serde(&ast)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
