use crate::quasiquote;
use proc_macro2::{Delimiter, Group, Ident, Punct, TokenStream, TokenTree};
use std::iter::Peekable;

type TokenIter = Peekable<proc_macro2::token_stream::IntoIter>;

#[derive(Debug, Clone)]
pub(crate) struct Parser(TokenIter);

#[derive(Debug, Clone)]
pub(crate) enum IterItem {
    Token(quasiquote::Token),
    Group(Group),
    Interpolation(quasiquote::Interpolation),
}

impl Parser {
    pub fn new(token_stream: TokenStream) -> Self {
        Self(token_stream.into_iter().peekable())
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
            IterItem::Interpolation(quasiquote::Interpolation::Binding(ident))
        } else if let TokenTree::Punct(ref punct) = token
              && punct.as_char() == '#'
              && let Some(TokenTree::Group(group)) = self.0.peek()
              && let Delimiter::Parenthesis | Delimiter::Brace = group.delimiter()
        {
            todo!()
        } else {
            match token {
                TokenTree::Ident(i) => IterItem::Token(quasiquote::Token::Ident(i)),
                TokenTree::Literal(l) => IterItem::Token(quasiquote::Token::Literal(l)),
                TokenTree::Punct(p) => IterItem::Token(quasiquote::Token::Punct(p)),
                TokenTree::Group(g) => IterItem::Group(g),
            }
        })
    }
}
