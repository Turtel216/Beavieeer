// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

// TODO 36

use crate::{
    ast::Program,
    lexer::{self, Lexer},
};

pub struct Parser<'p> {
    lexer: &'p mut Lexer<'p>,
    current: lexer::Token,
    peek: lexer::Token,
}

impl<'p> Parser<'p> {
    pub fn new(lexer: &'p mut Lexer<'p>) -> Self {
        let current = lexer.next_token();
        let peek = lexer.next_token();
        Self {
            lexer,
            current,
            peek,
        }
    }

    fn next_token(&mut self) -> () {
        self.current = self.peek.clone();
        self.peek = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Program {
        todo!()
    }
}
