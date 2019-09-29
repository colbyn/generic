#![allow(unused)]
extern crate proc_macro;

use quote::quote;
use syn::{
    Data,
    DataStruct,
    Fields,
    Field,
    Variant,
    FieldsUnnamed,
};
use generic_core::into::*;
use generic_core::value::*;



///////////////////////////////////////////////////////////////////////////////
// INTO-GENERIC ENTRYPOINT
///////////////////////////////////////////////////////////////////////////////

#[proc_macro_derive(IntoGeneric)]
pub fn into_generic_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse::<syn::DeriveInput>(input).expect("syn::parse failed");
    let type_name = ast.ident.clone();
    
    match ast.data.clone() {
        Data::Struct(DataStruct{fields: Fields::Named(named), ..}) => {
            let fields = named.named.into_iter().collect::<Vec<_>>();
            into_generic_struct(&type_name, &fields)
        }
        Data::Struct(DataStruct{fields: Fields::Unnamed(xs), ..}) => {
            into_generic_tuple_struct(&type_name, xs)
        }
        Data::Struct(DataStruct{fields: Fields::Unit, ..}) => {
            unimplemented!("Unit structs not yet supported")
        }
        Data::Enum(xs) => {
            let variants = xs.variants
                .into_iter()
                .collect::<Vec<_>>();
            into_generic_enum(&type_name, &variants)
        }
        Data::Union(_) => {
            unimplemented!("Union types not yet supported")
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// INTO-GENERIC - TUPLE-STRUCT
///////////////////////////////////////////////////////////////////////////////

fn into_generic_struct(type_name: &proc_macro2::Ident, fields: &[Field]) -> proc_macro::TokenStream {
    let field_conversions = fields
        .iter()
        .map(|field| {
            let field_ident = field.ident
                .clone()
                .expect("missing ident");
            quote! {
                let key = stringify!(#field_ident).to_owned();
                let value = IntoGeneric::into_generic(&self.#field_ident);
                map.insert(key, value);
            }
        })
        .collect::<Vec<_>>();
    let gen = quote! {
        impl IntoGeneric for #type_name {
            fn into_generic(&self) -> Value {
                use generic_core::into::*;
                use generic_core::value::*;

                let mut map: HashMap<String, Value> = std::collections::HashMap::new();
                #(#field_conversions)*
                Value::Struct(Struct {
                    type_name: String::from(stringify!(#type_name)),
                    data: map,
                })
            }
        }
    };
    gen.into()
}


///////////////////////////////////////////////////////////////////////////////
// INTO-GENERIC - TUPLE-STRUCT
///////////////////////////////////////////////////////////////////////////////

fn into_generic_tuple_struct(type_name: &proc_macro2::Ident, fields: FieldsUnnamed) -> proc_macro::TokenStream {
    let field_conversions = fields.unnamed
        .iter()
        .enumerate()
        .map(|(ix, _)| {
            let ix = syn::Index::from(ix);
            quote!{
                let value = IntoGeneric::into_generic(&self.#ix);
                vec.push(value);
            }
        })
        .collect::<Vec<_>>();
    let gen = quote! {
        impl IntoGeneric for #type_name {
            fn into_generic(&self) -> Value {
                use generic_core::into::*;
                use generic_core::value::*;

                let mut vec = Vec::<Value>::new();

                #(#field_conversions)*
                Value::TupleStruct(TupleStruct {
                    type_name: String::from(stringify!(#type_name)),
                    data: vec,
                })
            }
        }
    };
    gen.into()
}


///////////////////////////////////////////////////////////////////////////////
// INTO-GENERIC - ENUMS
///////////////////////////////////////////////////////////////////////////////

fn into_generic_enum(type_name: &proc_macro2::Ident, variants: &[Variant]) -> proc_macro::TokenStream {
    let arms = variants
        .iter()
        .map(|var| {
            let variant_name = &var.ident;
            match var.fields.clone() {
                Fields::Named(xs) => {
                    let (ident_binders, to_generics) = xs.named
                        .iter()
                        .map(|x| {
                            let ident = x.ident.clone().expect("todo - tuple structs");
                            let binder = quote!{
                                ref #ident,
                            };
                            let to_generic = quote!{
                                let key = stringify!(#ident).to_owned();
                                let value = IntoGeneric::into_generic(#ident);
                                map.insert(key, value);
                            };
                            (binder, to_generic)
                        })
                        .unzip::<_, _, Vec<_>, Vec<_>>();
                    quote! {
                        #type_name::#variant_name{#(#ident_binders)*} => {
                            let mut map = std::collections::HashMap::<String, Value>::new();

                            #(#to_generics)*
                            
                            Value::Variant(Variant::StructVariant {
                                type_name: stringify!(#type_name).to_owned(),
                                variant_name: stringify!(#variant_name).to_owned(),
                                data: map,
                            })
                        }
                    }
                },
                Fields::Unnamed(xs) => {
                    let (ident_binders, to_generics) = xs.unnamed
                        .iter()
                        .map(|_| {
                            let ident = format!("id_{}", rand::random::<u16>());
                            let ident = proc_macro2::Ident::new(&ident, proc_macro2::Span::call_site());
                            let binder = quote!{
                                ref #ident,
                            };
                            let to_generic = quote!{
                                let value = IntoGeneric::into_generic(#ident);
                                vec.push(value);
                            };
                            (binder, to_generic)
                        })
                        .unzip::<_, _, Vec<_>, Vec<_>>();
                    quote! {
                        #type_name::#variant_name(#(#ident_binders)*) => {
                            let mut vec = Vec::<Value>::new();

                            #(#to_generics)*
                            
                            Value::Variant(Variant::TupleVariant {
                                type_name: stringify!(#type_name).to_owned(),
                                variant_name: stringify!(#variant_name).to_owned(),
                                data: vec,
                            })
                        }
                    }
                },
                Fields::Unit => {
                    quote! {
                        #type_name::#variant_name => {
                            Value::Variant(Variant::UnitVariant{
                                type_name: stringify!(#type_name).to_owned(),
                                variant_name: stringify!(#variant_name).to_owned(),
                            })
                        }
                    }
                },
            }
        });
    let gen = quote! {
        impl IntoGeneric for #type_name {
            fn into_generic(&self) -> Value {
                use generic_core::into::*;
                use generic_core::value::*;

                match self {
                    #(#arms)*
                }
            }
        }
    };
    gen.into()
}

