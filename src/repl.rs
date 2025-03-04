// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

use crate::evaluator::builtins::new_builtins;
use crate::evaluator::env::Env;
use crate::evaluator::object::Object;
use crate::evaluator::Evaluator;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::cell::RefCell;
use std::error::Error;
use std::io::Write;
use std::io::{stdin, BufRead};
use std::rc::Rc;

const PROMPT: &str = ">> ";
const STAND_PRELUDE: &str = "
let fold = fun(f, init, lst) {
  if (len(lst) == 0) {
    init
  } else {
    let newInit = f(init, first(lst));
    fold(f, newInit, tail(lst));
  }
};
";

pub fn start(output: &mut dyn Write) {
    let mut line = String::new();
    let mut lang_input = String::new();
    let mut env = Env::from(new_builtins());

    env.set(
        String::from("print"),
        &Object::Builtin(-1, |args| {
            for arg in args {
                println!("{}", arg);
            }
            Object::Null
        }),
    );

    let mut evaluator = Evaluator::new(Rc::new(RefCell::new(env)));
    load_prelude(&mut evaluator);

    println!("Welcome to the Beavieeer REPL!");

    loop {
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();
        line.clear();
        lang_input.clear();

        let bytes_read = read_from_stdin(&mut line);
        if bytes_read == 0 {
            return; // End of input
        }

        let trimmed_line = line.trim();

        match trimmed_line {
            ":q" => {
                writeln!(output, "Exiting REPL. Goodbye!").unwrap();
                return;
            }
            line => {
                let mut parser = Parser::new(Lexer::new(&line));
                let program = parser.parse();
                let errors = parser.get_errors();

                if errors.len() > 0 {
                    for err in errors {
                        println!("{}", err);
                    }
                    continue;
                }

                if let Some(evaluated) = evaluator.eval(program) {
                    println!("{}\n", evaluated);
                }
            }
        }
    }
}

pub fn run_file(input: &String) -> () {
    let mut env = Env::from(new_builtins());

    env.set(
        String::from("print"),
        &Object::Builtin(-1, |args| {
            for arg in args {
                println!("{}", arg);
            }
            Object::Null
        }),
    );

    let mut evaluator = Evaluator::new(Rc::new(RefCell::new(env)));
    load_prelude(&mut evaluator);

    let mut parser = Parser::new(Lexer::new(&input));
    let program = parser.parse();
    let errors = parser.get_errors();

    if errors.len() > 0 {
        for err in errors {
            println!("{}", err);
        }
    }

    if let Some(evaluated) = evaluator.eval(program) {
        println!("{}\n", evaluated);
    }
}

// TODO: find a more efficient method. Dont call stdin().lock() on every function call
pub fn read_from_stdin(line: &mut String) -> usize {
    let mut input = stdin().lock();
    input.read_line(line).unwrap()
}

fn error_writting_to_stdout(err: &dyn Error) -> usize {
    panic!("Error when writting to stdout: {}", err);
}

fn error_reading_from_stdin(err: &dyn Error) -> usize {
    panic!("Error when reading from stdin: {}", err);
}

fn load_prelude(evaluator: &mut Evaluator) -> () {
    let mut parser = Parser::new(Lexer::new(STAND_PRELUDE));
    let program = parser.parse();
    let errors = parser.get_errors();

    if errors.len() > 0 {
        for err in errors {
            println!("Prelude Error: {}", err);
        }
    }

    if let Some(evaluated) = evaluator.eval(program) {
        println!("{}\n", evaluated);
    }
}
