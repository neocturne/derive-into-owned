#[macro_use]
extern crate derive_into_owned;

use std::borrow::Cow;

#[derive(IntoOwned, Borrowed)]
struct Array<'a> {
    a: [Cow<'a, str>; 2],
}

#[test]
fn array() {
    let val = Array {
        a: [Cow::Owned("str".to_owned()), Cow::Borrowed("str")],
    };
    let owned = val.into_owned();

    test_static(&owned);

    let borrowed = owned.borrowed();
    // owned cannot be moved while borrowed exists
    test_borrowed(&owned, borrowed);
}

fn test_static(_s: &Array<'static>) {}

fn test_borrowed<'b, 'a: 'b>(lives_longer: &Array<'a>, lives_less: Array<'b>) {
    drop(lives_less);
    #[allow(dropping_references)]
    drop(lives_longer);
}
