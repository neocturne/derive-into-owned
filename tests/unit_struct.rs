#![allow(unused)]

#[macro_use]
extern crate derive_into_owned;

#[derive(IntoOwned, Borrowed)]
struct Far;
