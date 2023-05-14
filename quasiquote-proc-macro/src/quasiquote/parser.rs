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
        use NonZeroUsize as Nth;
        let token = self.0.next()?;
        Some(if let TokenTree::Punct(ref punct) = token
            && punct.as_char() == '#'
            && let Some(TokenTree::Ident(ident)) = self.0.peek_nth(Nth::new(1).unwrap()).cloned()
        {
            self.0.consume(Nth::new(1).unwrap());
            IterItem::Interpolation(quasiquote::Interpolation::Binding(ident))
        } else if let TokenTree::Punct(ref punct) = token
              && punct.as_char() == '#'
              && let Some(TokenTree::Group(group)) = self.0.peek_nth(Nth::new(1).unwrap()).cloned()
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
