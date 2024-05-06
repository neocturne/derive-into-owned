#[macro_use]
extern crate derive_into_owned;

use std::borrow::Cow;

#[derive(IntoOwned, Borrowed)]
struct Tuple<'a, 'b, 'c> {
    a: (Cow<'a, str>, Option<Cow<'b, str>>),
    b: (Cow<'c, str>,),
}

#[test]
fn tuple() {
    let val = Tuple {
        a: (Cow::Owned("str".to_owned()), None),
        b: (Cow::Borrowed("str"),),
    };
    let owned = val.into_owned();

    test_static(&owned);

    let borrowed = owned.borrowed();
    // owned cannot be moved while borrowed exists
    test_borrowed(&owned, borrowed);
}

fn test_static(_s: &Tuple<'static, 'static, 'static>) {}

fn test_borrowed<'b, 'a: 'b>(lives_longer: &Tuple<'a, 'a, 'a>, lives_less: Tuple<'b, 'b, 'b>) {
    drop(lives_less);
    #[allow(dropping_references)]
    drop(lives_longer);
}
