#[macro_use]
extern crate quote;
extern crate proc_macro;

use std::iter::FromIterator;
use quote::ToTokens;
use proc_macro::TokenStream;
use syn::{Ident, WhereClause, TypeParam, Data, DataStruct, DataEnum, Fields, Field};

#[proc_macro_derive(Random)]
pub fn random(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output: proc_macro2::TokenStream = {
        let parsed = syn::parse2(input).unwrap();
        impl_random(&parsed)
    };

    proc_macro::TokenStream::from(output)
}

fn impl_random(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let type_params: Vec<Ident> = ast.generics.type_params()
        .map(|t| t.ident.clone())
        .collect();


    let where_clause = if type_params.is_empty() {
        ast.generics.where_clause.clone().map(WhereClause::into_token_stream)
    } else {
        let mut where_clause = ast.generics.where_clause.clone()
            .map(|x| quote!(#x,))
            .unwrap_or_else(|| quote!(where ));
        let generic_bounds = quote!(
           #(#type_params: autorand::Random),*
        );
        generic_bounds.to_tokens(&mut where_clause);
        Some(where_clause)
    };

    let body = match ast.data {
        Data::Struct(ref data) => expand_struct_random_body(data),
        Data::Enum(ref data) => expand_enum_random_body(data),
        Data::Union(_) => panic!("Random derive is not supported for Union types"),
    };

    let tokens = quote! {
        impl #impl_generics autorand::Random for #name #ty_generics #where_clause {
            fn random() -> Self {
                #body
            }
        }
    };

    //panic!("{}", tokens);

    tokens
}

fn expand_struct_random_body(data: &DataStruct) -> proc_macro2::TokenStream {
    let fields = expand_named_fields(data.fields.iter());
    quote!(
        Self {
            #(#fields),*
        }
    )
}

fn expand_named_fields<'a>(fields: impl Iterator<Item = &'a Field> + 'a) -> impl Iterator<Item = impl ToTokens> + 'a {
    fields.map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if f.ident.is_some() {
            quote! {
                #name: <#ty as autorand::Random>::random()
            }
        } else {
            quote! {
                <#ty as autorand::Random>::random()
            }
        }
    })
}

fn expand_enum_random_body(data: &DataEnum) -> proc_macro2::TokenStream {
    let constructors = data.variants.iter()
        .map(|v| {
            let name = &v.ident;
            match &v.fields {
                Fields::Named(fields) => {
                    let fields = expand_named_fields(fields.named.iter());
                    quote!(
                        Self::#name { #(#fields),* }
                    )
                },
                Fields::Unnamed(fields) => {
                    let fields = expand_named_fields(fields.unnamed.iter());
                    quote!(
                        Self::#name ( #(#fields),* )
                    )
                },
                Fields::Unit => {
                    quote!(Self::#name)
                }
            }
        });

    let matches = constructors.enumerate()
        .map(|(i, c)| quote!(#i => #c))
        .collect::<Vec<_>>();
    let count = matches.len();

    quote!(
        let variant = autorand::rand::random::<u16>() % #count;
        match variant {
            #(#matches),*
        }
    )
}