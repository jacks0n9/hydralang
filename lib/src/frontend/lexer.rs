//===----------------- lexer.rs - The file for lexical analysis --------------===//
//
// This source file is part of the hydralang open souce project
//
// Copyright (c) 2021 KittyBorgX and the englang project authors
// Licensed under Apache License v2.0 with Runtime Library Exception
//
// See https://github.com/KittyBorgX/hydralang/blob/main/LICENSE for license information
// See https://github.com/KittyBorgX/hydralang/blob/main/CONTRIBUTORS.md for the list of hydralang project authors
//
//===----------------------------------------------------------------------===//

use logos::{Lexer, Logos, Span};

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Brackets
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftCurly,

    #[token("}")]
    RightCurly,
    #[token("\"")]
    Quote,
    // Data types
    #[regex("[a-zA-Z]+")]
    Str,

    #[regex("[1-9]+")]
    Int,

    #[token("true")]
    #[token("false")]
    Boolean,

    // comments
    //
    //     #[regex("/\*([^*]|[\r\n]|(\*+([^*/]|[\r\n])))*\*+/")]
    //     Comment,
    //     //
    //     #[regex("[0.0-9.9]+")]
    //     Float,
    //
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
    // #[regex]
}
