use quote::{format_ident, quote};

use crate::helpers::{collect_segments, cow_field, generic_field, is_cow_alike};

#[derive(Debug)]
pub enum FieldKind {
    PlainCow(Box<FieldKind>),
    AssumedCow,
    OptField(Box<FieldKind>),
    IterableField(Box<FieldKind>),
    Array(Box<FieldKind>),
    JustMoved,
}
impl FieldKind {
    pub fn resolve(ty: &syn::Type) -> Self {
        match ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                let segments = collect_segments(path);

                if let Some(kind) = cow_field(&segments) {
                    FieldKind::PlainCow(Box::new(kind))
                } else if is_cow_alike(&segments) {
                    FieldKind::AssumedCow
                } else if let Some(kind) = generic_field(&segments, "std::option::Option") {
                    FieldKind::OptField(Box::new(kind))
                } else if let Some(kind) = generic_field(&segments, "std::vec::Vec") {
                    FieldKind::IterableField(Box::new(kind))
                } else {
                    FieldKind::JustMoved
                }
            }
            syn::Type::Array(syn::TypeArray { elem, .. }) => {
                FieldKind::Array(Box::new(FieldKind::resolve(elem)))
            }
            _ => FieldKind::JustMoved,
        }
    }

    pub fn move_or_clone_field(&self, var: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        use self::FieldKind::*;

        match self {
            PlainCow(inner) => {
                let next = format_ident!("val");
                let tokens = inner.move_or_clone_field(&quote! { #next });

                quote! {
                    {
                        let #next = ::std::borrow::Cow::into_owned(#var);
                        ::std::borrow::Cow::Owned(#tokens)
                    }
                }
            }
            AssumedCow => quote! { #var.into_owned() },
            OptField(inner) => {
                let next = format_ident!("val");
                let tokens = inner.move_or_clone_field(&quote! { #next });

                quote! { #var.map(|#next| #tokens) }
            }
            IterableField(inner) => {
                let next = format_ident!("x");
                let tokens = inner.move_or_clone_field(&quote! { #next });

                quote! { #var.into_iter().map(|#next| #tokens).collect() }
            }
            Array(inner) => {
                let next = format_ident!("x");
                let tokens = inner.move_or_clone_field(&quote! { #next });

                quote! { #var.map(|#next| #tokens) }
            }
            JustMoved => quote! { #var },
        }
    }

    pub fn borrow_or_clone(&self, var: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        use self::FieldKind::*;

        match self {
            PlainCow(_) => quote! { ::std::borrow::Cow::Borrowed(#var.as_ref()) },
            AssumedCow => quote! { #var.borrowed() },
            OptField(inner) => {
                let next = format_ident!("val");
                let tokens = inner.borrow_or_clone(&quote! { #next });

                quote! { #var.as_ref().map(|#next| #tokens) }
            }
            IterableField(inner) => {
                let next = format_ident!("x");
                let tokens = inner.borrow_or_clone(&quote! { #next });

                quote! { #var.iter().map(|#next| #tokens).collect() }
            }
            Array(inner) => {
                let next = format_ident!("x");
                let tokens = inner.borrow_or_clone(&quote! { #next });

                quote! { #var.each_ref().map(|#next| #tokens) }
            }
            JustMoved => quote! { #var.clone() },
        }
    }
}
