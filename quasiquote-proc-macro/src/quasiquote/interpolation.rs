use crate::quasiquote::QuasiQuote;
use proc_macro2::{Group, Ident, TokenStream};
use quote::quote;

#[derive(Debug, Clone)]
pub(crate) enum Interpolation {
    Binding(Ident),
    Expression(Group),
}

impl QuasiQuote for Interpolation {
    fn quasiquote(&self) -> TokenStream {
        match self {
            Self::Binding(binding) => {
                quote! {{&#binding}}
            }
            Self::Expression(group) => {
                let inner = group.stream();
                quote! { {#inner} }
            }
        }
    }
}
