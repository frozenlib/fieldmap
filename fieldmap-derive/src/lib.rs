#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use std::fmt::Display;
use structmeta::StructMeta;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::*;
use syn::*;
use utils::into_macro_output;

#[macro_use]
mod utils;

#[proc_macro_derive(Fields, attributes(fields))]
pub fn derive_field_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    into_macro_output(derive_field_map_core(parse_macro_input!(
        input as DeriveInput
    )))
}
fn derive_field_map_core(input: DeriveInput) -> Result<TokenStream> {
    let mut ts = TokenStream::new();
    if let Data::Struct(s) = &input.data {
        if let Some(item_id) = get_item_trait(&input.attrs)? {
            match &s.fields {
                Fields::Named(fields) => {
                    impl_field_map(&input, &item_id, &fields.named, &mut ts);
                }
                Fields::Unnamed(fields) => {
                    impl_field_map(&input, &item_id, &fields.unnamed, &mut ts);
                }
                Fields::Unit => {
                    impl_field_map(&input, &item_id, &Punctuated::new(), &mut ts);
                }
            }
            Ok(ts)
        } else {
            bail!(
                input.span(),
                "`#[fields(item = \"{{TraitName}}\"]` required."
            );
        }
    } else {
        bail!(input.span(), "`#[derive(Fields)]` supports only struct.");
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

#[derive(StructMeta)]
struct FieldsArgs {
    item: Expr,
}

fn get_item_trait(attrs: &[syn::Attribute]) -> Result<Option<Path>> {
    for attr in attrs {
        if attr.path.is_ident("fields") {
            let args: FieldsArgs = attr.parse_args()?;
            match args.item {
                Expr::Path(path) => return Ok(Some(path.path)),
                Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) => return Ok(Some(syn::parse_str::<Path>(&s.value())?)),
                _ => bail!(
                    attr.span(),
                    "item parameter must specify string literal or path."
                ),
            }
        }
    }
    Ok(None)
}
fn impl_field_map(
    input: &DeriveInput,
    item_id: &Path,
    fields: &Punctuated<Field, Comma>,
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
    let code = quote_spanned! { item_id.span() =>
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
}
impl Display for FieldKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FieldKey::Named(ident) => write!(f, "{}", trim_raw(&ident.to_string())),
            FieldKey::Unnamed(idx) => write!(f, "{}", idx),
        }
    }
}

fn trim_raw(s: &str) -> &str {
    if let Some(s) = s.strip_prefix("r#") {
        s
    } else {
        s
    }
}
