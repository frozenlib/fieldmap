#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::token::*;
use syn::Type;
use syn::*;

#[proc_macro_derive(Fields, attributes(fields))]
pub fn derive_field_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut ts = TokenStream::new();
    if let Data::Struct(s) = &input.data {
        if let Some((item_id, span)) = get_item_trait(&input.attrs) {
            match &s.fields {
                Fields::Named(fields) => {
                    impl_field_map(&input, &item_id, &fields.named, span, &mut ts);
                }
                Fields::Unnamed(fields) => {
                    impl_field_map(&input, &item_id, &fields.unnamed, span, &mut ts);
                }
                Fields::Unit => {
                    impl_field_map(&input, &item_id, &Punctuated::new(), span, &mut ts);
                }
            }
            ts.into()
        } else {
            panic!("`#[fields(item = \"{TraitName}\"]` required.");
        }
    } else {
        panic!("`#[derive(Fields)]` supports only struct.");
    }
}

#[proc_macro_derive(Field)]
pub fn derive_field(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut ts = TokenStream::new();
    if let Data::Struct(s) = &input.data {
        match &s.fields {
            Fields::Named(fields) => {
                impl_field_all(&input, &fields.named, &mut ts);
            }
            Fields::Unnamed(fields) => {
                impl_field_all(&input, &fields.unnamed, &mut ts);
            }
            Fields::Unit => {}
        }
        ts.into()
    } else {
        panic!("`#[derive(Field)]` supports only struct.");
    }
}

fn impl_field_all(input: &DeriveInput, fields: &Punctuated<Field, Comma>, ts: &mut TokenStream) {
    for (idx, field) in fields.iter().enumerate() {
        impl_field(input, idx, field, ts);
    }
}
fn to_member(idx: usize, field: &Field) -> Member {
    if let Some(id) = &field.ident {
        parse2(quote!(#id)).unwrap()
    } else {
        parse_str(&format!("{}", idx)).unwrap()
    }
}

fn impl_field(input: &DeriveInput, idx: usize, field: &Field, ts: &mut TokenStream) {
    let self_id = &input.ident;
    let (impl_g, self_g, impl_where) = input.generics.split_for_impl();

    let id = to_member(idx, field);
    let ty = &field.ty;

    let code = quote! {
        impl #impl_g ::fieldmap::Field<#ty> for #self_id #self_g #impl_where {
            #[inline]
            fn get(&self) -> &#ty {
                &self.#id
            }

            #[inline]
            fn get_mut(&mut self) -> &mut #ty {
                &mut self.#id
            }
        }
    };
    ts.extend(code);
}

fn get_item_trait(attrs: &[syn::Attribute]) -> Option<(Type, Span)> {
    for attr in attrs {
        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
            if meta_list.ident == "fields" {
                let nested = meta_list.nested;
                for meta in nested {
                    if let NestedMeta::Meta(Meta::NameValue(nv)) = meta {
                        if nv.ident == "item" {
                            if let Lit::Str(s) = nv.lit {
                                let t: Type = syn::parse_str(&s.value()).unwrap();
                                return Some((t, s.span()));
                            }
                        }
                    }
                    panic!("`fields` attribute must specify `#[fields(item = \"TraitName\")].");
                }
            }
        }
    }
    None
}
fn impl_field_map(
    input: &DeriveInput,
    item_id: &Type,
    fields: &Punctuated<Field, Comma>,
    span: Span,
    ts: &mut TokenStream,
) {
    let self_id = &input.ident;
    let (impl_g, self_g, impl_where) = input.generics.split_for_impl();
    let impl_gps = &input.generics.params;

    let mut arms_get = Vec::new();
    let mut arms_get_mut = Vec::new();
    let mut arms_name = Vec::new();
    let mut arms_find = Vec::new();
    for (idx, field) in fields.iter().enumerate() {
        let key = FieldKey::new(idx, field);
        let m = key.to_member();
        arms_get.push(quote!(#idx => Some(&self.#m)));
        arms_get_mut.push(quote!(#idx => Some(&mut self.#m)));

        let s = key.to_string();
        arms_name.push(quote!(#idx => Some(#s)));
        arms_find.push(quote!(#s => Some(#idx)));
    }

    let len = fields.len();
    let code = quote_spanned! { span =>
        impl #impl_g ::fieldmap::Fields for #self_id #self_g #impl_where {
            type Item = dyn #item_id;

            #[inline]
            fn len() -> usize {
               #len
            }
            #[inline]
            fn find(name: &str) -> Option<usize> {
                match name {
                    #(#arms_find,)*
                    _ => None,
                }
            }
            #[inline]
            fn name(idx: usize) -> Option<&'static str> {
                match idx {
                    #(#arms_name,)*
                    _ => None,
                }
            }
            #[inline]
            fn get(&self, idx: usize) -> ::core::option::Option<&Self::Item> {
                match idx {
                    #(#arms_get,)*
                    _ => None,
                }
            }
            #[inline]
            fn get_mut(&mut self, idx: usize) -> ::core::option::Option<&mut Self::Item> {
                match idx {
                    #(#arms_get_mut,)*
                    _ => None,
                }
            }
        }

        impl<'_a, #impl_gps> ::core::iter::IntoIterator for &'_a #self_id #self_g #impl_where {
            type Item = <::fieldmap::Iter<'_a, #self_id #self_g> as Iterator>::Item;
            type IntoIter = ::fieldmap::Iter<'_a, #self_id #self_g>;

            fn into_iter(self) -> Self::IntoIter {
                ::fieldmap::Fields::iter(self)
            }
        }

        impl<'_a, #impl_gps> ::core::iter::IntoIterator for &'_a mut #self_id #self_g #impl_where {
            type Item = <::fieldmap::IterMut<'_a, #self_id #self_g> as Iterator>::Item;
            type IntoIter = ::fieldmap::IterMut<'_a, #self_id #self_g>;

            fn into_iter(self) -> Self::IntoIter {
                ::fieldmap::Fields::iter_mut(self)
            }
        }
    };
    ts.extend(code);
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum FieldKey {
    Named(Ident),
    Unnamed(usize),
}

impl FieldKey {
    fn new(idx: usize, field: &Field) -> Self {
        if let Some(ident) = &field.ident {
            FieldKey::Named(ident.clone())
        } else {
            FieldKey::Unnamed(idx)
        }
    }
    fn to_member(&self) -> Member {
        match self {
            FieldKey::Named(ident) => Member::Named(ident.clone()),
            FieldKey::Unnamed(idx) => Member::Unnamed(parse_str(&format!("{}", idx)).unwrap()),
        }
    }
    fn to_string(&self) -> String {
        match self {
            FieldKey::Named(ident) => trim_raw(&ident.to_string()).to_string(),
            FieldKey::Unnamed(idx) => format!("{}", idx),
        }
    }
}

fn trim_raw(s: &str) -> &str {
    if s.starts_with("r#") {
        &s[2..]
    } else {
        s
    }
}
