//===----------------- run.rs - A file to accept the file name and run the .hy file--===//
//
// This source file is part of the hydralang open souce project
//
// Copyright (c) 2021 KittyBorgX and the englang project authors
// Licensed under Apache License v2.0 with Runtime Library Exception
//
// See https://github.com/KittyBorgX/hydralang/blob/main/LICENSE for license information
// See https://github.com/KittyBorgX/hydralang/blob/main/CONTRIBUTORS.md for the list of hydralang project authors
//
//===-----------------------------------------------------------------------------===//

mod lexer;
pub mod run {
    use super::lexer::Token;
    use logos::Logos;
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    pub fn run_lexer() {
        // Collect the arguments that are provided by the user
        let args: Vec<String> = env::args().collect();

        println!("{}", &args.len());
        let help = "To run a file use cargo run filename.hy";

        if &args.len() < &2 {
            println!("{}", help);
            std::process::exit(1);
        }

        // The filename will be the second element
        let filename = &args.get(1).unwrap();

        // Opens the file, panics if an error is returned
        let mut file = File::open(&filename.trim()).expect("No file found with that name");
        let mut contents = String::new();

        // Read the contents of the file and assign contents to it
        file.read_to_string(&mut contents).unwrap();

        for token in Token::lexer(&contents) {
            dbg!(token);
        }
    }
}
