#[path = "frontend/lexer.rs"]
mod lexer;
use lexer::Token;
use logos::Logos;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut lex = lexer::Token::lexer("Create ridiculously fast Lexers.");

    assert_eq!(lex.next(), Some(Token::Text));
    assert_eq!(lex.span(), 0..6);
    assert_eq!(lex.slice(), "Create");

    assert_eq!(lex.next(), Some(Token::Text));
    assert_eq!(lex.span(), 7..19);
    assert_eq!(lex.slice(), "ridiculously");

    assert_eq!(lex.next(), Some(Token::Fast));
    assert_eq!(lex.span(), 20..24);
    assert_eq!(lex.slice(), "fast");

    assert_eq!(lex.next(), Some(Token::Text));
    assert_eq!(lex.slice(), "Lexers");
    assert_eq!(lex.span(), 25..31);

    assert_eq!(lex.next(), Some(Token::Period));
    assert_eq!(lex.span(), 31..32);
    assert_eq!(lex.slice(), ".");

    assert_eq!(lex.next(), None);
}
