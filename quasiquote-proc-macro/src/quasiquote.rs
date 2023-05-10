#![allow(unused_imports)]

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use std::iter::Peekable;

type TokenIter = Peekable<proc_macro2::token_stream::IntoIter>;

trait QuasiQuote {
    fn quasiquote(&self) -> TokenStream;
}

#[derive(Debug, Clone)]
enum Token {
    Ident(Ident),
    Literal(Literal),
    Punct(Punct),
}

#[derive(Debug, Clone)]
enum Interpolation {
    Binding(Binding),
}

#[derive(Debug, Clone)]
enum IterItem {
    Token(Token),
    Group(Group),
    Interpolation(Interpolation),
}

#[derive(Debug, Clone)]
struct Binding(Ident);

#[derive(Debug, Clone)]
struct Parser(TokenIter);

pub fn expand(input: TokenStream) -> TokenStream {
    let parser = Parser::new(input);
    let mut output = TokenStream::new();
    for item in parser {
        let quasiquoted = match item {
            IterItem::Token(q) => q.quasiquote(),
            IterItem::Interpolation(interpolation) => interpolation.quasiquote(),
            IterItem::Group(g) => {
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

impl Parser {
    pub fn new(token_stream: TokenStream) -> Self {
        Self(token_stream.into_iter().peekable())
    }
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

impl QuasiQuote for Interpolation {
    fn quasiquote(&self) -> TokenStream {
        match self {
            Self::Binding(binding) => binding.quasiquote(),
        }
    }
}

impl QuasiQuote for Binding {
    fn quasiquote(&self) -> TokenStream {
        let inner = &self.0;
        quote! {
            {
                &#inner
            }
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
            ::quasiquote::proc_macro2::Group::new(#delimiter, #inner)
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
            Spacing::Alone => quote! {::quasiquote::proc_macro2::Spacing::Alone},
            Spacing::Joint => quote! {::quasiquote::proc_macro2::Spacing::Joint},
        }
    }
}

impl Iterator for Parser {
    type Item = IterItem;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.0.next()?;
        Some(if let TokenTree::Punct(ref punct) = token
            && punct.as_char() == '#'
            && let Some(TokenTree::Ident(ident)) = self.0.peek().cloned()
        {
            let _ = self.0.next();
            IterItem::Interpolation(Interpolation::Binding(Binding(ident)))
        } else if let TokenTree::Punct(ref punct) = token
              && punct.as_char() == '#'
              && let Some(TokenTree::Group(group)) = self.0.peek()
              && let Delimiter::Parenthesis | Delimiter::Brace = group.delimiter()
        {
            todo!()
        } else {
            match token {
                TokenTree::Ident(i) => IterItem::Token(Token::Ident(i)),
                TokenTree::Literal(l) => IterItem::Token(Token::Literal(l)),
                TokenTree::Punct(p) => IterItem::Token(Token::Punct(p)),
                TokenTree::Group(g) => IterItem::Group(g),
            }
        })
    }
}
