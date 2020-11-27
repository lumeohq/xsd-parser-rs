use proc_macro2::{Span, TokenStream};
use quote::quote;

pub fn serde(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &ast.ident;
    let struct_name_literal = &ast.ident.to_string();

    let variants = match &ast.data {
        syn::Data::Enum(data_enum) => data_enum
            .variants
            .iter()
            .filter(|variant| &variant.ident.to_string() != "__Unknown__")
            .map(|variant| {
                let subtype = match &variant.fields {
                    syn::Fields::Unnamed(fields) => {
                        &fields
                            .unnamed
                            .first()
                            .ok_or_else(|| {
                                syn::Error::new_spanned(
                                    fields,
                                    "One unnamed field per variant is expected",
                                )
                            })?
                            .ty
                    }
                    fields => {
                        return Err(syn::Error::new_spanned(
                            fields,
                            "Only unnamed fields are supported",
                        ));
                    }
                };

                Ok((&variant.ident, subtype))
            })
            .collect::<syn::Result<Vec<_>>>()?,
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "This macro can only be used on enums",
            ));
        }
    };

    let ser_variants = variants
        .iter()
        .map(|(ident, _subtype)| {
            quote! {
                #struct_name::#ident(val) => val.to_string(),
            }
        })
        .collect::<TokenStream>();

    let de_variants = variants
        .iter()
        .map(|(ident, subtype)| {
            quote! {
                if let Ok(de) = s.parse::<#subtype>() {
                    return Ok(#struct_name::#ident(de))
                }
            }
        })
        .collect::<TokenStream>();

    Ok(quote! {
        impl YaSerialize for #struct_name {
            fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
                utils::yaserde::serialize(self, #struct_name_literal, writer, |s| {
                    match s {
                        #ser_variants
                        #struct_name::__Unknown__(_) => "".to_string()
                    }
                })
            }

            fn serialize_attributes(
                &self,
                attributes: Vec<xml::attribute::OwnedAttribute>,
                namespace: xml::namespace::Namespace,
            ) -> Result<
                (
                    Vec<xml::attribute::OwnedAttribute>,
                    xml::namespace::Namespace,
                ),
                std::string::String,
            > {
                Ok((attributes, namespace))
            }
        }

        impl YaDeserialize for #struct_name {
            fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
                utils::yaserde::deserialize(reader, |s| {
                    #de_variants
                    Ok(#struct_name::__Unknown__(s.to_string()))
                })
            }
        }
    })
}
