#[macro_use]
extern crate derive_into_owned;

use std::borrow::Cow;

#[allow(dead_code, clippy::redundant_allocation)]
#[derive(IntoOwned, Borrowed)]
struct TestTypes<'a> {
    a: Box<Cow<'a, str>>,
    b: Box<i32>,
    c: Option<i32>,
    d: Vec<i32>,
    e: Option<Box<i32>>,
    f: Option<Box<Box<i32>>>,
    g: Box<Option<()>>,
    h: String,
    i: (String, String),
    j: (),
    k: Option<(i32, String)>,
    l: Option<(i32, Vec<String>, Cow<'a, str>)>,
    m: Box<(i32, String, Vec<Cow<'a, str>>)>,
    n: Vec<(i32, Option<String>, Option<Cow<'a, str>>)>,
    o: ((), ()),
    p: (String, (String, (String, String))),
    #[allow(clippy::type_complexity)]
    q: (String, (String, (String, Box<Cow<'a, str>>))),
}
