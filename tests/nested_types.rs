#[macro_use]
extern crate derive_into_owned;

use std::borrow::Cow;

#[derive(IntoOwned, Borrowed)]
struct NestedTypes<'a> {
    a: Vec<Option<Cow<'a, Option<Cow<'a, str>>>>>,
}

#[test]
fn triple_cow() {
    let val = NestedTypes {
        a: vec![Some(Cow::Owned(Some(Cow::Borrowed("str"))))],
    };
    let owned = val.into_owned();

    test_static(&owned);

    let borrowed = owned.borrowed();
    // owned cannot be moved while borrowed exists
    test_borrowed(&owned, borrowed);
}

fn test_static(_s: &NestedTypes<'static>) {}

fn test_borrowed<'b, 'a: 'b>(lives_longer: &NestedTypes<'a>, lives_less: NestedTypes<'b>) {
    drop(lives_less);
    #[allow(dropping_references)]
    drop(lives_longer);
}
