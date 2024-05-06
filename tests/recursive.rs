use std::borrow::Cow;

#[macro_use]
extern crate derive_into_owned;

#[derive(IntoOwned, Borrowed)]
enum Rec<'a> {
    Rec(Option<Box<Rec<'a>>>),
    Cow(Cow<'a, str>),
}

#[test]
fn rec() {
    let val = Rec::Rec(Some(Box::new(Rec::Cow(Cow::Borrowed("str")))));
    let owned = val.into_owned();

    test_static(&owned);

    let borrowed = owned.borrowed();
    // owned cannot be moved while borrowed exists
    test_borrowed(&owned, borrowed);
}

fn test_static(_s: &Rec<'static>) {}

fn test_borrowed<'b, 'a: 'b>(lives_longer: &Rec<'a>, lives_less: Rec<'b>) {
    drop(lives_less);
    #[allow(dropping_references)]
    drop(lives_longer);
}
