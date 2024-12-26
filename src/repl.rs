// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

use std::error::Error;
use std::io::BufRead;
use std::io::Write;

use crate::lexer::Lexer;
use crate::lexer::TokenType;

const PROMPT: &str = ">> ";

pub fn start(input: &mut dyn BufRead, output: &mut dyn Write) {
    let mut line = String::new();

    loop {
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        line.clear();
        let bytes_read = input
            .read_line(&mut line)
            .unwrap_or_else(|err| error_reading_from_stdin(&err));
        if bytes_read == 0 {
            return; // End of input
        }

        let trimmed_line = line.trim();

        // Exit condition: If the user types `:q`
        if trimmed_line == ":q" {
            writeln!(output, "Exiting REPL. Goodbye!").unwrap_or_else(|err| {
                error_writting_to_stdout(&err);
            });
            return;
        }

        let mut l = Lexer::new(&line);
        loop {
            let tok = l.next_token();
            if tok._type == TokenType::Eof {
                break;
            }
            writeln!(output, "{:?}", tok).unwrap_or_else(|err| {
                error_writting_to_stdout(&err);
            });
        }
    }
}

fn error_writting_to_stdout(err: &dyn Error) -> usize {
    panic!("Error when writting to stdout: {}", err);
}

fn error_reading_from_stdin(err: &dyn Error) -> usize {
    panic!("Error when reading from stdin: {}", err);
}
