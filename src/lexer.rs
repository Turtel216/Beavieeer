// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

/// The lexer responsible for tokenizing the input string.
///
/// The `Lexer` struct scans through the input and produces tokens
/// based on the Beavieeer programming language's syntax.
use crate::token::Token;

/// Represents the lexical analyzer (lexer) for tokenizing input.
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,      // Current position in input (points to current character)
    next_pos: usize, // Next reading position in input
    ch: u8,          // Current character being examined
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer` instance from the given input string.
    ///
    /// This initializes the lexer and reads the first character.
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            pos: 0,
            next_pos: 0,
            ch: 0,
        };

        lexer.read_char();
        lexer
    }

    /// Reads the next character from the input and advances position markers.
    fn read_char(&mut self) {
        if self.next_pos >= self.input.len() {
            self.ch = 0; // End of file (EOF)
        } else {
            self.ch = self.input.as_bytes()[self.next_pos];
        }
        self.pos = self.next_pos;
        self.next_pos += 1;
    }

    /// Peeks at the next character without advancing the lexer.
    fn nextch(&mut self) -> u8 {
        if self.next_pos >= self.input.len() {
            0 // EOF
        } else {
            self.input.as_bytes()[self.next_pos]
        }
    }

    /// Checks if the next character matches the given byte.
    fn nextch_is(&mut self, ch: u8) -> bool {
        self.nextch() == ch
    }

    /// Skips over whitespace characters (spaces and tabs) and C++ style comments.
    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' => self.read_char(),
                b'/' => {
                    if self.nextch() == b'/' {
                        // Skip the current '/' and the next '/'
                        self.read_char();
                        self.read_char();

                        // Continue reading until end of line or EOF
                        while self.ch != b'\n' && self.ch != 0 {
                            self.read_char();
                        }
                    } else {
                        // Not a comment, just a single '/' character
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    /// Retrieves the next token from the input.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }
            b'>' => {
                if self.nextch_is(b'=') {
                    self.read_char();
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            }
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'[' => Token::Lbracket,
            b']' => Token::Rbracket,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b':' => Token::Colon,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => return self.consume_identifier(),
            b'0'..=b'9' => return self.consume_number(),
            b'"' => return self.consume_string(),
            b'\n' => {
                if self.nextch_is(b'\n') {
                    Token::Blank
                } else {
                    self.read_char();
                    return self.next_token();
                }
            }
            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();
        tok
    }

    /// Consumes an identifier or keyword from the input and returns the corresponding token.
    fn consume_identifier(&mut self) -> Token {
        let start_pos = self.pos;

        loop {
            match self.ch {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.read_char(),
                _ => break,
            }
        }

        let literal = &self.input[start_pos..self.pos];
        match literal {
            "fun" => Token::Func,
            "let" => Token::Let,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(String::from(literal)),
        }
    }

    /// Consumes a number from the input and returns it as an integer token.
    fn consume_number(&mut self) -> Token {
        let start_pos = self.pos;

        loop {
            match self.ch {
                b'0'..=b'9' => self.read_char(),
                _ => break,
            }
        }

        let literal = &self.input[start_pos..self.pos];
        Token::Int(literal.parse::<i64>().unwrap())
    }

    /// Consumes a string literal from the input, including handling closing quotes.
    fn consume_string(&mut self) -> Token {
        self.read_char();
        let start_pos = self.pos;

        loop {
            match self.ch {
                b'"' | 0 => {
                    let literal = &self.input[start_pos..self.pos];
                    self.read_char();
                    return Token::String(literal.to_string());
                }
                _ => self.read_char(),
            }
        }
    }
}
