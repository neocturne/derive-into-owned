# derive-into-owned

Proof of concept Rust procedural macro for deriving methods like:

```
import std::borrow::Cow;

struct Foo<'a> {
	field: Cow<'a, str>,
}

impl<'a> Foo<'a> {
	/// This method would be derived using #[derive(IntoOwned)]
	fn into_owned(self) -> Foo<'static> {
		Foo {
			field: Cow::Owned(self.field.into_owned()),
		}
	}
}
```

Currently it is just an edited version of [deep-clone-derive](https://github.com/asajeffrey/deep-clone/blob/master/deep-clone-derive/lib.rs) example but supports:

 * [tuple structs](./blob/tests/tuple_struct.rs)
 * normal [structs](./blob/tests/struct.rs)
 * enums with tuple variants [tuple enums](./blob/tests/simple_enum.rs)
 * `IntoOwned` alike fields (actually assumes all fields with types with lifetimes are `IntoOwned` alike)
 * options of Cow types `Option<Cow<'a, str>>`
 * options of Cow-like types `Option<Foo<'a>>`

## Limitations

Currently it will fail miserably for at least but not limited to:

 * borrowed fields like `&'a str`

Using with incompatible types results in not so understandable error messages. For example, given a struct:

```
#[derive(IntoOwned)]
struct Foo<'a> {
	field: &'a str,
}
```

The compiler error will be:

```
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
 --> tests/does_not_compile.rs:4:10
  |
4 | #[derive(IntoOwned)]
  |          ^^^^^^^^^
  |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the impl at 4:10...
 --> tests/does_not_compile.rs:4:10
  |
4 | #[derive(IntoOwned)]
  |          ^^^^^^^^^
note: ...so that reference does not outlive borrowed content
 --> tests/does_not_compile.rs:4:10
  |
4 | #[derive(IntoOwned)]
  |          ^^^^^^^^^
  = note: but, the lifetime must be valid for the static lifetime...
note: ...so that expression is assignable (expected Foo<'static>, found Foo<'_>)
 --> tests/does_not_compile.rs:4:10
  |
4 | #[derive(IntoOwned)]
  |          ^^^^^^^^^
error: aborting due to previous error(s)
```


## Types with lifetimes

If your struct has a field with type `Bar<'a>` then `Bar` is assumed to have a method `fn into_owned(self) -> Bar<'static>`.
