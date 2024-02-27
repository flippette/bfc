#![no_std]

use bfc_lexer::{Lexer, Source, Token};

/// a brainfuck instruction
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Instruction<'a> {
    /// move the data ptr left
    MoveL,
    /// move the data ptr right
    MoveR,

    /// increment the current addr
    Incr,
    /// decrement the current addr
    Decr,

    /// print the current addr's value
    Print,
    /// store a value into current addr
    Store,

    /// loop until current addr is 0
    Loop(Parser<'a>),
}

/// a brainfuck parser as an [`Iterator`] over [`Instruction`]s.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// create a new [`Parser`] from a [`Lexer`]
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    /// create a new [`Parser`] from a byte slice
    pub fn parse(src: Source<'a>) -> Self {
        Self::new(Lexer::new(src))
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Instruction<'a>> {
        loop {
            return Some(match self.lexer.next()? {
                Token::ChevronL => Instruction::MoveL,
                Token::ChevronR => Instruction::MoveR,
                Token::Plus => Instruction::Incr,
                Token::Minus => Instruction::Decr,
                Token::Dot => Instruction::Print,
                Token::Comma => Instruction::Store,
                Token::BracketL => {
                    let loop_lexer = self.lexer.clone();

                    // skip our own lexer past the current loop
                    let mut loop_count = 1;
                    while loop_count != 0 {
                        match self.lexer.next()? {
                            Token::BracketL => loop_count += 1,
                            Token::BracketR => loop_count -= 1,
                            _ => continue,
                        }
                    }

                    Instruction::Loop(Self::new(loop_lexer))
                }
                Token::BracketR => return None,
                Token::Comment(_) => continue,
            });
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Instruction, Parser};

    macro_rules! assert_opt {
        ($opt:expr) => {
            assert!($opt.is_none());
        };
        ($opt:expr, $val:expr $(,)?) => {
            assert_eq!($opt, Some($val))
        };
    }

    macro_rules! get_loop {
        ($parser:expr) => {
            match $parser.next() {
                Some(Instruction::Loop(parser)) => parser,
                other => {
                    panic!("expected Some(Instruction::Loop), got {other:?}")
                }
            }
        };
    }

    macro_rules! repeat {
        ($n:expr, $s:stmt) => {
            for _ in 0..$n {
                $s
            }
        };
    }

    #[test]
    fn hello() {
        let mut parser = Parser::parse(
            b">++++++++[<+++++++++>-]<.
              >++++[<+++++++>-]<+.
              +++++++..
              +++.",
        );

        assert_opt!(parser.next(), Instruction::MoveR);
        repeat!(8, assert_opt!(parser.next(), Instruction::Incr));

        let mut loop1 = get_loop!(parser);
        assert_opt!(loop1.next(), Instruction::MoveL);
        repeat!(9, assert_opt!(loop1.next(), Instruction::Incr));
        assert_opt!(loop1.next(), Instruction::MoveR);
        assert_opt!(loop1.next(), Instruction::Decr);

        assert_opt!(parser.next(), Instruction::MoveL);
        assert_opt!(parser.next(), Instruction::Print);

        assert_opt!(parser.next(), Instruction::MoveR);
        repeat!(4, assert_opt!(parser.next(), Instruction::Incr));

        let mut loop2 = get_loop!(parser);
        assert_opt!(loop2.next(), Instruction::MoveL);
        repeat!(7, assert_opt!(loop2.next(), Instruction::Incr));
        assert_opt!(loop2.next(), Instruction::MoveR);
        assert_opt!(loop2.next(), Instruction::Decr);

        assert_opt!(parser.next(), Instruction::MoveL);
        assert_opt!(parser.next(), Instruction::Incr);
        assert_opt!(parser.next(), Instruction::Print);

        repeat!(7, assert_opt!(parser.next(), Instruction::Incr));
        repeat!(2, assert_opt!(parser.next(), Instruction::Print));

        repeat!(3, assert_opt!(parser.next(), Instruction::Incr));
        assert_opt!(parser.next(), Instruction::Print);
    }

    #[test]
    fn cat() {
        let mut parser = Parser::parse(b",[.,]");

        assert_opt!(parser.next(), Instruction::Store);

        let mut loop1 = get_loop!(parser);
        assert_opt!(loop1.next(), Instruction::Print);
        assert_opt!(loop1.next(), Instruction::Store);
        assert_opt!(loop1.next());

        assert_opt!(parser.next());
    }
}
