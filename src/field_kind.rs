use quote::{format_ident, quote};

use crate::helpers::{collect_segments, cow_field, generic_field, is_cow_alike};

#[derive(Debug)]
pub enum FieldKind {
    PlainCow(Box<FieldKind>),
    AssumedCow,
    OptField(Box<FieldKind>),
    IterableField(Box<FieldKind>),
    Box(Box<FieldKind>),
    Array(Box<FieldKind>),
    Tuple(Vec<FieldKind>),
    JustMoved,
}
impl FieldKind {
    pub fn resolve(ty: &syn::Type) -> Self {
        let field_kind = match ty {
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
                } else if let Some(kind) = generic_field(&segments, "std::boxed::Box") {
                    FieldKind::Box(Box::new(kind))
                } else {
                    FieldKind::JustMoved
                }
            }
            syn::Type::Array(syn::TypeArray { elem, .. }) => {
                FieldKind::Array(Box::new(FieldKind::resolve(elem)))
            }
            syn::Type::Tuple(syn::TypeTuple { elems, .. }) => {
                if elems.is_empty() {
                    // Unit
                    FieldKind::JustMoved
                } else {
                    FieldKind::Tuple(elems.iter().map(FieldKind::resolve).collect())
                }
            }
            _ => FieldKind::JustMoved,
        };

        // Optimization to shortcut to JustMoved for containers that contain no
        // references (thus generating a single move / clone instead of handling
        // all inner fields)
        if field_kind.is_static() {
            return FieldKind::JustMoved;
        }

        field_kind
    }

    fn is_static(&self) -> bool {
        match self {
            FieldKind::PlainCow(_) => false,
            FieldKind::AssumedCow => false,
            FieldKind::OptField(inner) => inner.is_static(),
            FieldKind::IterableField(inner) => inner.is_static(),
            FieldKind::Box(inner) => inner.is_static(),
            FieldKind::Array(inner) => inner.is_static(),
            FieldKind::Tuple(elems) => elems.iter().all(FieldKind::is_static),
            FieldKind::JustMoved => true,
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
            Box(inner) => {
                let tokens = inner.move_or_clone_field(&quote! { (*#var) });

                quote! { ::std::boxed::Box::new(#tokens) }
            }
            Array(inner) => {
                let next = format_ident!("x");
                let tokens = inner.move_or_clone_field(&quote! { #next });

                quote! { #var.map(|#next| #tokens) }
            }
            Tuple(fields) => {
                let next = format_ident!("val");
                let fields = fields.iter().enumerate().map(|(index, field)| {
                    let index = syn::Index::from(index);
                    field.move_or_clone_field(&quote! { #next.#index })
                });
                quote! {
                    {
                        let #next = #var;
                        ( #(#fields),* , )
                    }
                }
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
            Box(inner) => {
                let tokens = inner.borrow_or_clone(&quote! { #var.as_ref() });

                quote! { ::std::boxed::Box::new(#tokens) }
            }
            Array(inner) => {
                let next = format_ident!("x");
                let tokens = inner.borrow_or_clone(&quote! { #next });

                quote! { #var.each_ref().map(|#next| #tokens) }
            }
            Tuple(fields) => {
                let next = format_ident!("val");
                let fields = fields.iter().enumerate().map(|(index, field)| {
                    let index = syn::Index::from(index);
                    field.borrow_or_clone(&quote! { (&#next.#index) })
                });
                quote! {
                    {
                        let #next = #var;
                        ( #(#fields),* , )
                    }
                }
            }
            JustMoved => quote! { #var.clone() },
        }
    }
}
