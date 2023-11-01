#![allow(dead_code)]

use crate::tokens::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    loc: usize,
}

impl Lexer {
    pub fn new(source: Vec<char>) -> Self {
        Lexer { source, loc: 0 }
    }

    pub fn tokenise(mut self) -> Vec<Token> {
        while self.loc < self.source.len() {
            println!("{}", self.source[self.loc]);
            self.loc += 1;
        }

        vec![Token::new(TokenKind::Semicolon, String::from(";"))]
    }

    pub fn peek(&self) -> char {
        self.source[self.loc]
    }

    pub fn consume(mut self) -> char {
        let res = self.source[self.loc];
        self.loc += 1;
        res
    }
}
