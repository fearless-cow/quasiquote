Provides a wrapper around quote! that allows interpolating arbitrary expressions.

Expected syntax is identical to what's used for quote!, except that a new interpolation pattern
is allowed.
# Examples
With quote:
```rust
use quote::quote;
use proc_macro2::TokenStream;
use syn::{Field, Member};

pub fn expand_getter(field: &Field) -> TokenStream {
    let ident = &field.ident;
    let member = Member::Named(ident.as_ref().cloned().unwrap());
    let ty = &field.ty;
    quote! {
        pub fn #ident(&self) -> #ty {
            &self.#member
        }
    }
}
```
With quasiquote:
```rust
use quasiquote::quasiquote;
use proc_macro2::TokenStream;
use syn::{Field, Member};

pub fn expand_getter(field: &Field) -> TokenStream {
    let member = Member::Named(field.ident.as_ref().cloned().unwrap());
    quasiquote! {
        pub fn #{&field.ident}(&self) -> #{&field.ty} {
            &self.#member
        }
    }
}
```
# Not yet implemented
* interpolation of expressions inside of repetitions
