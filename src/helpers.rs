use crate::field_kind::FieldKind;

pub fn has_lifetime_arguments(segments: &[&syn::PathSegment]) -> bool {
    if let Some(syn::PathArguments::AngleBracketed(generics)) =
        segments.last().map(|x| &x.arguments)
    {
        generics
            .args
            .iter()
            .any(|f| matches!(f, syn::GenericArgument::Lifetime(_)))
    } else {
        false
    }
}

fn type_hopefully_is(segments: &[&syn::PathSegment], expected: &str) -> bool {
    let expected = expected
        .split("::")
        .map(|x| quote::format_ident!("{}", x))
        .collect::<Vec<_>>();
    if segments.len() > expected.len() {
        return false;
    }

    let expected = expected.iter().collect::<Vec<_>>();
    let segments = segments.iter().map(|x| &x.ident).collect::<Vec<_>>();

    for len in 0..expected.len() {
        if segments[..] == expected[expected.len() - len - 1..] {
            return true;
        }
    }

    false
}

pub fn cow_field(segments: &[&syn::PathSegment]) -> Option<FieldKind> {
    if !type_hopefully_is(segments, "std::borrow::Cow") {
        return None;
    }

    let syn::PathSegment {
        arguments: syn::PathArguments::AngleBracketed(data),
        ..
    } = segments.last().expect("last segment")
    else {
        return None;
    };

    if data.args.len() != 2 {
        return None;
    }
    let syn::GenericArgument::Lifetime(_) = &data.args[0] else {
        return None;
    };
    let syn::GenericArgument::Type(arg_type) = &data.args[1] else {
        return None;
    };

    Some(FieldKind::resolve(arg_type))
}

pub fn is_cow_alike(segments: &[&syn::PathSegment]) -> bool {
    matches!(
        segments.last().map(|x| &x.arguments),
        Some(syn::PathArguments::AngleBracketed(_))
    ) && has_lifetime_arguments(segments)
}

pub fn collect_segments(path: &syn::Path) -> Vec<&syn::PathSegment> {
    path.segments.iter().collect::<Vec<_>>()
}

/// Checks for a given type with a single generic type argument
///
/// Examples for such types are [Option<T>] and [Vec<T>].
pub fn generic_field(segments: &[&syn::PathSegment], type_name: &str) -> Option<FieldKind> {
    if !type_hopefully_is(segments, type_name) {
        return None;
    }

    let syn::PathSegment {
        arguments: syn::PathArguments::AngleBracketed(data),
        ..
    } = segments.last().expect("last segment")
    else {
        return None;
    };

    if data.args.len() != 1 {
        return None;
    }
    let syn::GenericArgument::Type(arg_type) = &data.args[0] else {
        return None;
    };

    Some(FieldKind::resolve(arg_type))
}
