#![allow(dead_code)]

pub enum LiteralKind {
    String,
    Int,
}

pub enum TokenKind {
    Identifier,
    Literal { kind: LiteralKind },
    Semicolon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
}

pub struct Token {
    kind: TokenKind,
    value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        Token { kind, value }
    }
}
