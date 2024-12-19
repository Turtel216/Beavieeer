// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals,
    Ident(String),
    Int(String),

    // Operators
    Assign,
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
            TokenType::Assign => write!(f, "Token Assign"),
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
            TokenType::Int(v) => write!(f, "Token Int: {v}"),
        }
    }
}

pub struct Lexer<'l> {
    input: &'l String,
    position: usize,
    read_position: usize,
    chr: char,
}

impl<'l> Lexer<'l> {
    pub fn new(input: &'l String) -> Self {
        let result = Self {
            input,
            position: 0,
            read_position: 0,
            chr: input
                .chars()
                .nth(0)
                .unwrap_or_else(|| panic!("Error: Empty file")),
        };

        return result;
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

        self.chr = ch;
        return self.chr;
    }

    fn next_token(&mut self) -> Token {
        self.skip_white_space();
        let tok = match self.chr {
            '=' => {
                self.read_char();
                Token::new(TokenType::Assign, String::from('='))
            }
            '+' => {
                self.read_char();
                Token::new(TokenType::Plus, String::from('+'))
            }
            ',' => {
                self.read_char();
                Token::new(TokenType::Comma, String::from(','))
            }
            ';' => {
                self.read_char();
                Token::new(TokenType::Semicolon, String::from(';'))
            }
            '(' => {
                self.read_char();
                Token::new(TokenType::LParen, String::from('('))
            }
            ')' => {
                self.read_char();
                Token::new(TokenType::RParen, String::from(')'))
            }
            '{' => {
                self.read_char();
                Token::new(TokenType::LBrace, String::from('{'))
            }
            '}' => {
                self.read_char();
                Token::new(TokenType::RBrace, String::from('}'))
            }
            '\0' => Token::new(TokenType::Eof, String::from("")),
            _ => {
                if self.chr.is_alphabetic() {
                    let literal = self.read_identifier();
                    let type_ = self.look_up_ident(&literal);
                    Token::new(type_, literal)
                } else if self.chr.is_numeric() {
                    let literal = self.read_number();
                    let type_ = TokenType::Int(literal.clone());
                    Token::new(type_, literal)
                } else {
                    Token::new(TokenType::Illegal, String::from(self.chr))
                }
            }
        };

        return tok;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.chr.is_alphabetic() {
            _ = self.read_char();
        }

        return self.input[position..self.position].to_string();
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.chr.is_numeric() {
            _ = self.read_char();
        }

        return self.input[position..self.position].to_string();
    }

    fn look_up_ident(&self, input: &str) -> TokenType {
        match input {
            "fun" => TokenType::Function,
            "let" => TokenType::Let,
            _ => TokenType::Ident(String::from(input)),
        }
    }

    fn skip_white_space(&mut self) -> () {
        while self.chr.is_whitespace() {
            _ = self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::lexer::TokenType;

    #[test]
    fn test_next_token() {
        let input = String::from(
            "let five = 5;
let ten = 10;
let add = fun(x, y) {
x + y;
};
let result = add(five, ten);",
        );

        let output = [
            TokenType::Let,
            TokenType::Ident(String::from("five")),
            TokenType::Assign,
            TokenType::Int(String::from("5")),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident(String::from("ten")),
            TokenType::Assign,
            TokenType::Int(String::from("10")),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident(String::from("add")),
            TokenType::Assign,
            TokenType::Function,
            TokenType::LParen,
            TokenType::Ident(String::from("x")),
            TokenType::Comma,
            TokenType::Ident(String::from("y")),
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Ident(String::from("x")),
            TokenType::Plus,
            TokenType::Ident(String::from("y")),
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident(String::from("result")),
            TokenType::Assign,
            TokenType::Ident(String::from("add")),
            TokenType::LParen,
            TokenType::Ident(String::from("five")),
            TokenType::Comma,
            TokenType::Ident(String::from("ten")),
            TokenType::RParen,
            TokenType::Semicolon,
            TokenType::Eof,
        ];

        let mut lexer = Lexer::new(&input);

        for test_case in output {
            let token = lexer.next_token();
            assert_eq!(token._type, test_case);
        }
    }
}
