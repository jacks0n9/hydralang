//===----------------- main.rs - The main rust file in this project ----------===//
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

#[path = "frontend/run.rs"]
mod run;

fn main() {
    run::run::run_lexer();
}

//
// #[path = "frontend/lexer.rs"]
// mod lexer;
// use lexer::Token;
// use logos::Logos;
// use std::fs::File;
// use std::io::prelude::*;
//
//
// fn main() -> std::io::Result<()> {
//     let mut file = File::open("./tests/lex.hy")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(for token in Token::lexer(&contents) {
//         dbg!(token);
//     })
// }
