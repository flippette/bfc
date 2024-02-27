#![no_std]

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    combinator::map,
    Err, IResult,
};

pub type Source<'a> = &'a [u8];
pub type Result<'a> = IResult<Source<'a>, Token<'a>>;

/// a brainfuck source code token
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token<'a> {
    ChevronL,
    ChevronR,
    Plus,
    Minus,
    Dot,
    Comma,
    BracketL,
    BracketR,
    Comment(Source<'a>),
}

/// an [`Iterator`] over [`Token`]s
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Lexer<'a> {
    src: Source<'a>,
}

impl<'a> Token<'a> {
    /// parse a single token from the beginning of a [`Source`]
    pub fn parse(src: Source<'a>) -> Result {
        alt((
            map(tag(b"<"), |_| Token::ChevronL),
            map(tag(b">"), |_| Token::ChevronR),
            map(tag(b"+"), |_| Token::Plus),
            map(tag(b"-"), |_| Token::Minus),
            map(tag(b"."), |_| Token::Dot),
            map(tag(b","), |_| Token::Comma),
            map(tag(b"["), |_| Token::BracketL),
            map(tag(b"]"), |_| Token::BracketR),
            map(take_till1(|b| b"<>+-.,[]".contains(&b)), Token::Comment),
        ))(src)
    }
}

impl<'a> Lexer<'a> {
    pub fn new(src: Source<'a>) -> Self {
        Self { src }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        match Token::parse(self.src) {
            Ok((src, tok)) => {
                self.src = src;
                Some(tok)
            }
            Err(Err::Error(src) | Err::Failure(src)) => {
                self.src = src.input;
                None
            }
            Err(Err::Incomplete(_)) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    macro_rules! assert_opt {
        ($opt:expr) => {
            assert!($opt.is_none());
        };
        ($opt:expr, $val:expr $(,)?) => {
            assert_eq!($opt, Some($val))
        };
    }

    #[test]
    fn symbols_only() {
        let mut lexer = Lexer::new(b"<>+-.,[]");

        assert_opt!(lexer.next(), Token::ChevronL);
        assert_opt!(lexer.next(), Token::ChevronR);
        assert_opt!(lexer.next(), Token::Plus);
        assert_opt!(lexer.next(), Token::Minus);
        assert_opt!(lexer.next(), Token::Dot);
        assert_opt!(lexer.next(), Token::Comma);
        assert_opt!(lexer.next(), Token::BracketL);
        assert_opt!(lexer.next(), Token::BracketR);
        assert_opt!(lexer.next());
    }

    #[test]
    fn comment_only() {
        let mut lexer = Lexer::new(b"hello");
        assert_opt!(lexer.next(), Token::Comment(b"hello"));
        assert_opt!(lexer.next());
    }

    #[test]
    fn mixed() {
        let mut lexer = Lexer::new(b"[]<>hello>");
        assert_opt!(lexer.next(), Token::BracketL);
        assert_opt!(lexer.next(), Token::BracketR);
        assert_opt!(lexer.next(), Token::ChevronL);
        assert_opt!(lexer.next(), Token::ChevronR);
        assert_opt!(lexer.next(), Token::Comment(b"hello"));
        assert_opt!(lexer.next(), Token::ChevronR);
        assert_opt!(lexer.next());
    }
}
