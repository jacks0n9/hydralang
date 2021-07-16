#[path = "frontend/lexer.rs"]
mod lexer;
use lexer::Token;
use logos::Logos;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./tests/lex.hy")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(for token in Token::lexer(&contents) {
        dbg!(token);
    })
}
