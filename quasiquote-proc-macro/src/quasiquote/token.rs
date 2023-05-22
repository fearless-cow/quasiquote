use crate::quasiquote::QuasiQuote;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream};
use quote::quote;

#[derive(Debug, Clone)]
pub(crate) enum Token {
    Ident(Ident),
    Literal(Literal),
    Punct(Punct),
}

impl QuasiQuote for Token {
    fn quasiquote(&self) -> TokenStream {
        match self {
            Self::Ident(i) => i.quasiquote(),
            Self::Literal(l) => l.quasiquote(),
            Self::Punct(p) => p.quasiquote(),
        }
    }
}

impl QuasiQuote for Ident {
    fn quasiquote(&self) -> TokenStream {
        let s = self.to_string();
        quote! {
            ::quasiquote::proc_macro2::Ident::new(#s, ::quasiquote::proc_macro2::Span::call_site())
        }
    }
}

impl QuasiQuote for Literal {
    fn quasiquote(&self) -> TokenStream {
        let s = self.to_string();
        quote! {
            <::quasiquote::proc_macro2::Literal as ::std::str::FromStr>::from_str(#s).unwrap()
        }
    }
}

impl QuasiQuote for Punct {
    fn quasiquote(&self) -> TokenStream {
        let c = self.as_char();
        let spacing = self.spacing().quasiquote();
        quote! {
            ::quasiquote::proc_macro2::Punct::new(#c, #spacing)
        }
    }
}

impl QuasiQuote for Group {
    fn quasiquote(&self) -> TokenStream {
        let inner = self.stream();
        let delimiter = self.delimiter().quasiquote();
        quote! {{
            let inner = #inner;
            ::quasiquote::proc_macro2::Group::new(#delimiter, inner)
        }}
    }
}

impl QuasiQuote for Delimiter {
    fn quasiquote(&self) -> TokenStream {
        match self {
            Self::Parenthesis => quote! {::quasiquote::proc_macro2::Delimiter::Parenthesis},
            Self::Bracket => quote! {::quasiquote::proc_macro2::Delimiter::Bracket},
            Self::Brace => quote! {::quasiquote::proc_macro2::Delimiter::Brace},
            Self::None => quote! {::quasiquote::proc_macro2::Delimiter::None},
        }
    }
}

impl QuasiQuote for Spacing {
    fn quasiquote(&self) -> TokenStream {
        match self {
            Self::Alone => quote! {::quasiquote::proc_macro2::Spacing::Alone},
            Self::Joint => quote! {::quasiquote::proc_macro2::Spacing::Joint},
        }
    }
}
