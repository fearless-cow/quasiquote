use quasiquote::{quasiquote, quote::quote};
use std::fmt;

fn compare_strings<A, B>(a: A, b: B)
where
    A: AsRef<str> + fmt::Display,
    B: AsRef<str> + fmt::Display,
{
    if a.as_ref()
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<String>()
        != b.as_ref()
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect::<String>()
    {
        panic!("strings do not match:\n{}\n{}", a, b)
    }
}

#[test]
fn pass_through() {
    let a = quasiquote! {
        fn ferris<T>(_: T) -> T {
            let a = [1, 2, 3];
            let b = {
                1 + 2 + (3 + 4)
            };
        }
    }
    .to_string();
    let b = stringify! {
        fn ferris<T>(_: T) -> T {
            let a = [1, 2, 3];
            let b = {
                1 + 2 + (3 + 4)
            };
        }
    };
    compare_strings(a, b);
}

#[test]
fn interpolate_binding() {
    let i = quote! { 2 };
    let a = quasiquote! {
        fn f() {
            let x = #i * 2;
        }
    }
    .to_string();
    let b = stringify! {
        fn f() {
            let x = 2 * 2;
        }
    };
    compare_strings(a, b);
}

#[test]
fn interpolate_expression() {
    let quoted = quasiquote! {
        let x = #{ 1 + 1 };
    }
    .to_string();
    let expected = stringify! {
        let x = 2;
    };
    compare_strings(quoted, expected);
}

#[cfg(any())]
fn interpolate_iterator() {
    let a = quote! { 1 };
    let b = quote! { 2 };
    let c = quote! { 3 };
    let array = [a, b, c];
    let iter = array.iter();
    let quoted = quasiquote! {
        let array = [#(#iter),*];
    };
}
