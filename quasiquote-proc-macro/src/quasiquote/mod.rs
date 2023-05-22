mod interpolation;
mod parser;
mod token;

use interpolation::Interpolation;
use parser::Parser;
use proc_macro2::{Group, TokenStream};
use quote::{quote, ToTokens};
use token::Token;

trait QuasiQuote {
    fn quasiquote(&self) -> TokenStream;
}

pub fn expand(input: TokenStream) -> TokenStream {
    let parser = Parser::new(input);
    let mut output = TokenStream::new();
    for item in parser {
        let quasiquoted = match item {
            parser::IterItem::Token(q) => q.quasiquote(),
            parser::IterItem::Interpolation(interpolation) => interpolation.quasiquote(),
            parser::IterItem::Group(g) => {
                let inner = expand(g.stream());
                let group = Group::new(g.delimiter(), inner);
                group.quasiquote()
            }
        };
        quote! {
            {
                let x = #quasiquoted;
                ::quasiquote::quote::ToTokens::to_tokens(&x, &mut _ts);
            };
        }
        .to_tokens(&mut output);
    }
    quote! {
       {
            let mut _ts = ::quasiquote::proc_macro2::TokenStream::new();
            #output
            _ts
       }
    }
}
