use crate::quasiquote::QuasiQuote;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

#[derive(Debug, Clone)]
pub(crate) enum Interpolation {
    Binding(Ident),
}

impl QuasiQuote for Interpolation {
    fn quasiquote(&self) -> TokenStream {
        match self {
            Self::Binding(binding) => {
                quote! {{&#binding}}
            }
        }
    }
}
