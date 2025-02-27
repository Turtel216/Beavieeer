// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

/// Represents the different types of tokens in the Beavieeer programming language.
///
/// This enumeration covers all possible tokens, including operators, delimiters,
/// keywords, literals, and special tokens.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Represents an unrecognized or illegal token.
    Illegal,
    /// Represents a blank line or whitespace.
    Blank,
    /// Represents the end of the input stream.
    Eof,

    // Identifiers + literals
    /// Represents an identifier (variable or function name).
    Ident(String),
    /// Represents an integer literal.
    Int(i64),
    /// Represents a string literal.
    String(String),
    /// Represents a boolean literal (`true` or `false`).
    Bool(bool),

    // Statements
    /// Represents an assignment (`=`) operator.
    Assign,
    /// Represents the `if` keyword.
    If,
    /// Represents the `else` keyword.
    Else,

    // Operators
    /// Represents the addition (`+`) operator.
    Plus,
    /// Represents the subtraction (`-`) operator.
    Minus,
    /// Represents the logical NOT (`!`) operator.
    Bang,
    /// Represents the multiplication (`*`) operator.
    Asterisk,
    /// Represents the division (`/`) operator.
    Slash,

    /// Represents the equality comparison (`==`).
    Equal,
    /// Represents the inequality comparison (`!=`).
    NotEqual,
    /// Represents the less-than comparison (`<`).
    LessThan,
    /// Represents the less-than-or-equal comparison (`<=`).
    LessThanEqual,
    /// Represents the greater-than comparison (`>`).
    GreaterThan,
    /// Represents the greater-than-or-equal comparison (`>=`).
    GreaterThanEqual,

    // Delimiters
    /// Represents a comma (`,`).
    Comma,
    /// Represents a colon (`:`).
    Colon,
    /// Represents a semicolon (`;`).
    Semicolon,
    /// Represents a left parenthesis (`(`).
    Lparen,
    /// Represents a right parenthesis (`)`).
    Rparen,
    /// Represents a left brace (`{`).
    Lbrace,
    /// Represents a right brace (`}`).
    Rbrace,
    /// Represents a left bracket (`[`).
    Lbracket,
    /// Represents a right bracket (`]`).
    Rbracket,

    // Reserved keywords
    /// Represents the `fun` keyword for defining functions.
    Func,
    /// Represents the `let` keyword for variable declarations.
    Let,
    /// Represents the `return` keyword.
    Return,
}
