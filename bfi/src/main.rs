use std::{
    env, fs,
    io::{self, IsTerminal, Read, Write},
    process,
};

use bfc_parser::{Instruction, Parser};
use crossterm::terminal;

fn main() -> io::Result<()> {
    assert!(
        io::stdout().is_terminal(),
        "stdout is not a tty, terminating!"
    );

    let path = env::args().nth(1).expect("program path should be provided");
    let program = fs::read(path)?;
    let mut parser = Parser::parse(&program);
    let mut mem = vec![0u8; 30_000];
    let mut ptr = 0;

    terminal::enable_raw_mode()?;
    let ret = run(
        &mut parser,
        &mut mem,
        &mut ptr,
        &mut io::stdout(),
        &mut io::stdin(),
    )?;
    terminal::disable_raw_mode()?;

    process::exit(ret as i32)
}

fn run(
    parser: &mut Parser,
    mem: &mut [u8],
    ptr: &mut usize,
    out: &mut impl Write,
    inp: &mut impl Read,
) -> io::Result<u8> {
    for inst in parser {
        match inst {
            Instruction::MoveL => {
                *ptr = if *ptr == 0 { mem.len() - 1 } else { *ptr - 1 }
            }
            Instruction::MoveR => {
                *ptr = if *ptr == mem.len() - 1 { 0 } else { *ptr + 1 }
            }
            Instruction::Incr => mem[*ptr] = mem[*ptr].wrapping_add(1),
            Instruction::Decr => mem[*ptr] = mem[*ptr].wrapping_sub(1),
            Instruction::Print => {
                write!(out, "{}", mem[*ptr] as char)?;
                out.flush()?; // is raw mode not working???
            }
            Instruction::Store => inp.read_exact(&mut mem[*ptr..*ptr + 1])?,
            Instruction::Loop(parser) => {
                while mem[*ptr] != 0 {
                    run(&mut parser.clone(), mem, ptr, out, inp)?;
                }
            }
        }
    }

    Ok(mem[*ptr])
}
