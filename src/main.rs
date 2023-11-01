use std::{env, fs};

use lexer::Lexer;
use tokens::Token;

mod lexer;
mod tokens;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("didn't provide a file path")
    }

    let file_path = &args[1];

    let source = match fs::read_to_string(file_path) {
        Err(why) => panic!("{}", why),
        Ok(source) => source,
    };

    let lexer: Lexer = Lexer::new(source.chars().collect());

    let tokens: Vec<Token> = lexer.tokenise();
}
