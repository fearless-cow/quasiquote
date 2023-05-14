use crate::quasiquote;
use itertools::{Itertools, MultiPeek};
use proc_macro2::{Delimiter, Group, Ident, Punct, TokenStream, TokenTree};
use std::iter;
use std::num::NonZeroUsize;

type TokenIter = proc_macro2::token_stream::IntoIter;

#[derive(Debug, Clone)]
struct InnerIterator(MultiPeek<TokenIter>);

#[derive(Debug, Clone)]
pub(crate) struct Parser(InnerIterator);

#[derive(Debug, Clone)]
pub(crate) enum IterItem {
    Token(quasiquote::Token),
    Group(Group),
    Interpolation(quasiquote::Interpolation),
}

impl InnerIterator {
    fn next(&mut self) -> Option<TokenTree> {
        self.0.next()
    }

    fn peek_nth(&mut self, n: NonZeroUsize) -> Option<&TokenTree> {
        self.0.reset_peek();
        for _ in 1..n.get() {
            self.0.peek()?;
        }
        self.0.peek()
    }

    fn consume(&mut self, n: NonZeroUsize) {
        for _ in 1..n.get() {
            match self.0.next() {
                Some(_) => continue,
                None => break,
            }
        }
    }
}

impl From<quasiquote::Token> for IterItem {
    fn from(value: quasiquote::Token) -> Self {
        Self::Token(value)
    }
}

impl From<Group> for IterItem {
    fn from(value: Group) -> Self {
        Self::Group(value)
    }
}

impl From<quasiquote::Interpolation> for IterItem {
    fn from(value: quasiquote::Interpolation) -> Self {
        Self::Interpolation(value)
    }
}

#[allow(clippy::let_unit_value)]
impl Parser {
    pub fn new(token_stream: TokenStream) -> Self {
        let inner = InnerIterator(token_stream.into_iter().multipeek());
        Self(inner)
    }
}

impl Iterator for Parser {
    type Item = IterItem;

    fn next(&mut self) -> Option<Self::Item> {
        use quasiquote::Interpolation;
        use quasiquote::Token;
        use NonZeroUsize as Nth;
        Some(
            match (
                self.0.next()?,
                self.0.peek_nth(Nth::new(1).unwrap()).cloned(),
                self.0.peek_nth(Nth::new(2).unwrap()).cloned(),
                self.0.peek_nth(Nth::new(3).unwrap()).cloned(),
            ) {
                (TokenTree::Punct(punct), Some(TokenTree::Ident(binding)), ..)
                    if punct.as_char() == '#' =>
                {
                    self.0.consume(Nth::new(1).unwrap());
                    Interpolation::Binding(binding).into()
                }
                (TokenTree::Literal(literal), ..) => Token::Literal(literal).into(),
                (TokenTree::Punct(punct), ..) => Token::Punct(punct).into(),
                (TokenTree::Ident(ident), ..) => Token::Ident(ident).into(),
                (TokenTree::Group(group), ..) => group.into(),
            },
        )
    }
}
