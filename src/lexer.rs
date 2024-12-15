// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

use std::fmt;

pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals,
    Ident(String),
    Int(String),

    // Operators
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}

pub struct Token {
    _type: TokenType,
    literal: String,
}

impl Token {
    fn new(_type: TokenType, literal: String) -> Self {
        Self { _type, literal }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Illegal => write!(f, "Token Illegal"),
            TokenType::Eof => write!(f, "Token Eof"),
            TokenType::Plus => write!(f, "Token Plus"),
            TokenType::Comma => write!(f, "Token Comma"),
            TokenType::Semicolon => write!(f, "Token Semicolon"),
            TokenType::LParen => write!(f, "Token LParen"),
            TokenType::RParen => write!(f, "Token RParen"),
            TokenType::LBrace => write!(f, "Token LBrace"),
            TokenType::RBrace => write!(f, "Token RBrace"),
            TokenType::Function => write!(f, "Token Function"),
            TokenType::Let => write!(f, "Token Let"),
            TokenType::Ident(v) => write!(f, "Token Ident: {v}"),
            TokenType::Int(v) => write!(f, "Token Ident: {v}"),
        }
    }
}

pub struct Lexer<'l> {
    input: &'l String,
    position: usize,
    read_position: usize,
}

impl<'l> Lexer<'l> {
    pub fn new(input: &'l String) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
        }
    }

    fn read_char(&mut self) -> char {
        let ch;
        if self.read_position >= self.input.len() {
            ch = '\0';
        } else {
            ch = self
                .input
                .chars()
                .nth(self.read_position)
                .unwrap_or_else(|| {
                    panic!(
                        "Error in read_char(). No character at index {}",
                        self.read_position
                    )
                });
        }
        self.position = self.read_position;
        self.read_position += 1;

        return ch;
    }

    fn next_token(&mut self) -> Token {
        let tok = match self.read_char() {
            '=' => Token::new(TokenType::Plus, String::from('=')),
            _ => todo!(),
        };

        return tok;
    }
}
