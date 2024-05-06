#![allow(dead_code)]

#[macro_use]
extern crate derive_into_owned;

use std::borrow::{self, Cow};

#[derive(IntoOwned, Borrowed)]
struct Foo<'a>(Cow<'a, str>);

#[derive(IntoOwned, Borrowed)]
struct FooExtraFields<'a>(u32, Cow<'a, str>, bool, Vec<bool>);

#[derive(IntoOwned, Borrowed)]
struct Bar<'a>(::std::borrow::Cow<'a, str>);

#[derive(IntoOwned, Borrowed)]
struct Car<'a>(std::borrow::Cow<'a, str>);

#[derive(IntoOwned, Borrowed)]
struct Dar<'a>(borrow::Cow<'a, str>);

#[test]
fn tuple_struct() {
    let non_static_string: String = "foobar".to_string();

    let thing = Foo(Cow::Borrowed(&non_static_string));
    let owned = thing.into_owned();

    accepts_only_static(&owned);

    let borrowed = owned.borrowed();
    // owned cannot be moved while borrowed exists
    test_borrowed(&owned, borrowed);
}

fn accepts_only_static(_static_foo: &Foo<'static>) {}

fn test_borrowed<'b, 'a: 'b>(lives_longer: &Foo<'a>, lives_less: Foo<'b>) {
    drop(lives_less);
    #[allow(dropping_references)]
    drop(lives_longer);
}
