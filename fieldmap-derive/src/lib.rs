#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::token::*;
use syn::Type;
use syn::*;

#[proc_macro_derive(FieldMap, attributes(field_map))]
pub fn derive_field_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let mut ts = TokenStream::new();
    if let Data::Struct(s) = input.data {
        match &s.fields {
            Fields::Named(fields) => {
                impl_field_map_entry_all(ident, &fields.named, &mut ts);
            }
            Fields::Unnamed(fields) => {
                impl_field_map_entry_all(ident, &fields.unnamed, &mut ts);
            }
            Fields::Unit => {}
        }
        if let Some((item_id, span)) = get_item_trait(&input.attrs) {
            match &s.fields {
                Fields::Named(fields) => {
                    impl_field_map(ident, &item_id, &fields.named, span, &mut ts);
                }
                Fields::Unnamed(fields) => {
                    impl_field_map(ident, &item_id, &fields.unnamed, span, &mut ts);
                }
                Fields::Unit => {
                    impl_field_map(ident, &item_id, &Punctuated::new(), span, &mut ts);
                }
            }
        }
        proc_macro::TokenStream::from(ts)
    } else {
        panic!("Deriveing `FieldMap` supports only struct.");
    }
}

fn impl_field_map_entry_all(
    struct_ident: &Ident,
    fields: &Punctuated<Field, Comma>,
    ts: &mut TokenStream,
) {
    for (idx, field) in fields.iter().enumerate() {
        impl_field_map_entry(struct_ident, idx, field, ts);
    }
}
fn to_id(idx: usize, field: &Field) -> proc_macro2::TokenStream {
    if let Some(id) = &field.ident {
        quote! { #id }
    } else {
        quote! { #idx }
    }
}

fn impl_field_map_entry(self_id: &Ident, idx: usize, field: &Field, ts: &mut TokenStream) {
    let id = to_id(idx, field);
    let ty = &field.ty;

    let code = quote! {
        impl ::fieldmap::FieldMapEntry<#ty> for #self_id {
            #[inline]
            fn get(&self) -> &#ty {
                &self.#id
            }

            #[inline]
            fn get_mut(&mut self) -> &mut #ty {
                &mut self.#id
            }

            #[inline]
            fn replace(&mut self, value: #ty) -> #ty {
                ::core::mem::replace(&mut self.#id, value)
            }
        }
    };
    ts.extend(code);
}

fn get_item_trait(attrs: &[syn::Attribute]) -> Option<(Type, Span)> {
    for attr in attrs {
        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
            if meta_list.ident == "field_map" {
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
                    panic!("`fieldmap` attribute must specify `#[fieldmap(item = \"TraitName\")].");
                }
            }
        }
    }
    None
}
fn impl_field_map(
    self_id: &Ident,
    item_id: &Type,
    fields: &Punctuated<Field, Comma>,
    span: Span,
    ts: &mut TokenStream,
) {
    let mut arms_get = Vec::new();
    let mut arms_get_mut = Vec::new();
    for (idx, field) in fields.iter().enumerate() {
        let id = to_id(idx, field);
        arms_get.push(quote!(#idx => Some(&self.#id)));
        arms_get_mut.push(quote!(#idx => Some(&mut self.#id)));
    }

    let len = fields.len();
    let code = quote_spanned! { span =>
        impl ::fieldmap::FieldMap for #self_id {
            type Item = dyn #item_id;

            #[inline]
            fn len(&self) -> usize {
               #len
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

        impl #self_id {
            pub fn iter(&self) -> ::fieldmap::FieldMapIter<Self> {
                ::fieldmap::FieldMapIter::new(self)
            }
            pub fn iter_mut(&mut self) -> ::fieldmap::FieldMapIterMut<Self> {
                ::fieldmap::FieldMapIterMut::new(self)
            }
        }

        impl<'a> ::core::iter::IntoIterator for &'a #self_id {
            type Item = &'a (dyn #item_id + 'static);
            type IntoIter = ::fieldmap::FieldMapIter<'a, #self_id>;

            fn into_iter(self) -> Self::IntoIter {
                ::fieldmap::FieldMapIter::new(self)
            }
        }

        impl<'a> ::core::iter::IntoIterator for &'a mut #self_id {
            type Item = &'a mut (dyn #item_id + 'static);
            type IntoIter = ::fieldmap::FieldMapIterMut<'a, #self_id>;

            fn into_iter(self) -> Self::IntoIter {
                ::fieldmap::FieldMapIterMut::new(self)
            }
        }
    };
    ts.extend(code);
}
