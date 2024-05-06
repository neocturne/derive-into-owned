#[macro_use]
extern crate derive_into_owned;

use std::borrow::Cow;

// Note: Borrowed currently can't be derived for a type with multiple dependent
// lifetimes - there is no easy way to determine whether multiple lifetimes
// on an inner type are used for separate fields, or nested like here. At the
// moment, it is assumed that the lifetimes are indepentent.

#[derive(IntoOwned)]
struct TripleCow<'a, 'b, 'c> {
    a: Cow<'a, Cow<'b, Cow<'c, str>>>,
}

#[test]
fn triple_cow() {
    let inner1 = Cow::Owned("str".to_owned());
    let inner2 = Cow::Borrowed(&inner1);
    let val = TripleCow {
        a: Cow::Borrowed(&inner2),
    };
    let owned = val.into_owned();

    test_static(&owned);
}

fn test_static(_s: &TripleCow<'static, 'static, 'static>) {}
