#[path = "frontend/lexer.rs"]
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

        match args.len() {
            0 => {
                println!("Pog, you didnt give an argument");
            }, 
            
            1 => {
                
        // The filename will be the second element
        let filename = &args[1];
        
        // Opens the file, panics if an error is returned
        let mut file = File::open(&filename.trim()).expect("No file found with that name");
        let mut contents = String::new();

        // Read the contents of the file and assign contents to it
        file.read_to_string(&mut contents).unwrap();

        for token in Token::lexer(&contents) {
            dbg!(token);
        }

            }

            _ => {
            // show a help message
                println!("Welp");
            }
        }

    }
}
