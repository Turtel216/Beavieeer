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
use std::collections::HashMap;
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

/// Starts a Read-Eval-Print Loop (REPL) for the Beavieeer language.
///
/// This function runs an interactive session that:
/// - Reads user input from stdin
/// - Evaluates the input as Beavieeer code
/// - Prints the result to the provided output
/// - Loops until the user inputs ":q" or EOF is reached
///
/// It also provides special commands:
/// - `:q` - Quit the REPL
/// - `:info` - List all available built-in functions
/// - `:info <function>` - Show documentation for a specific built-in function
/// - `:help` - Display help information for REPL commands
///
/// # Arguments
///
/// * `output` - A mutable reference to a type that implements the `Write` trait,
///              used for displaying prompts and results
///
/// # Examples
///
/// ```
/// use std::io;
/// use beavieeer::repl::start_repl;
/// let mut stdout = io::stdout();
/// start_repl(&mut stdout);
/// ```
#[inline]
pub fn start_repl(output: &mut dyn Write) {
    let mut line = String::new();
    let mut lang_input = String::new();
    let mut env = Env::from(new_builtins());
    let buildin_doc = get_buildin_doc();
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
    writeln!(output, "Welcome to the Beavieeer REPL!").unwrap();
    writeln!(
        output,
        "Type :q to quit, :info <function> to get function documentation"
    )
    .unwrap();

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

        // Check if the line starts with ":info "
        if trimmed_line.starts_with(":info ") {
            let func_name = trimmed_line.trim_start_matches(":info ").trim();
            match buildin_doc.get(func_name) {
                Some(doc) => writeln!(output, "Function: {}\n{}", func_name, doc).unwrap(),
                None => writeln!(output, "No documentation found for '{}'", func_name).unwrap(),
            }
            continue;
        }

        match trimmed_line {
            ":q" => {
                writeln!(output, "Exiting REPL. Goodbye!").unwrap();
                return;
            }
            ":info" => {
                writeln!(output, "Usage: :info <function_name>").unwrap();
                writeln!(output, "Available functions:").unwrap();

                let mut function_list = String::new();
                for (i, name) in buildin_doc.keys().enumerate() {
                    if i > 0 {
                        function_list.push_str(", ");
                    }
                    function_list.push_str(name);
                }
                writeln!(output, "{}", function_list).unwrap();
            }
            ":help" => {
                writeln!(output, "Available commands:").unwrap();
                writeln!(output, "  :q                - Quit the REPL").unwrap();
                writeln!(output, "  :info             - List available functions").unwrap();
                writeln!(
                    output,
                    "  :info <function>  - Show documentation for a specific function"
                )
                .unwrap();
                writeln!(output, "  :help             - Show this help message").unwrap();
            }
            line => {
                let mut parser = Parser::new(Lexer::new(line));
                let program = parser.parse();
                let errors = parser.get_errors();
                if !errors.is_empty() {
                    for err in errors {
                        writeln!(output, "{}", err).unwrap();
                    }
                    continue;
                }
                if let Some(evaluated) = evaluator.eval(program) {
                    writeln!(output, "{}", evaluated).unwrap();
                }
            }
        }
    }
}

#[inline]
pub fn run_file(input: &str) {
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

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse();
    let errors = parser.get_errors();

    if !errors.is_empty() {
        for err in errors {
            println!("{}", err);
        }
    }

    if let Some(evaluated) = evaluator.eval(program) {
        println!("{}\n", evaluated);
    }
}

// TODO: find a more efficient method. Dont call stdin().lock() on every function call
#[inline]
pub fn read_from_stdin(line: &mut String) -> usize {
    let mut input = stdin().lock();
    input.read_line(line).unwrap()
}

#[inline]
fn load_prelude(evaluator: &mut Evaluator) {
    let mut parser = Parser::new(Lexer::new(STAND_PRELUDE));
    let program = parser.parse();
    let errors = parser.get_errors();

    if !errors.is_empty() {
        for err in errors {
            println!("Prelude Error: {}", err);
        }
    }

    if let Some(evaluated) = evaluator.eval(program) {
        println!("{}\n", evaluated);
    }
}
/// Returns a HashMap containing documentation for all built-in functions.
///
/// This function creates and populates a HashMap where:
/// - Keys are the names of built-in functions as Strings
/// - Values are documentation strings that include:
///   - A description of what the function does
///   - The function's signature in the format "InputType -> OutputType"
///
/// The documentation covers several categories of built-in functions:
/// - I/O functions (print, read, readFile, writeFile)
/// - List operations (len, first, last, tail, get, map, filter, reverse)
/// - Functional utilities (fold)
/// - String utilities (lowercase, uppercase, trim, parseNumber, replaceString, replaceN, explode)
///
/// # Returns
///
/// A `HashMap<String, String>` mapping function names to their documentation.
#[inline]
fn get_buildin_doc() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // I/O
    map.insert(
        String::from("print"),
        String::from("Prints a value to the console.\nString -> Null"),
    );
    map.insert(
        String::from("read"),
        String::from("Read a value from the console.\nNothing -> String"),
    );
    map.insert(
        String::from("readFile"),
        String::from("Read the contents of a file\nString -> String"),
    );
    map.insert(
        String::from("writeFile"),
        String::from("Writes to a file the given content.\nCreates a file if it does not exist\nString -> Null"),
    );

    // List operations
    map.insert(
        String::from("len"),
        String::from("Returns the length of a given list.\nList -> Number"),
    );
    map.insert(
        String::from("first"),
        String::from("Returns the first element of a given list.\nList -> ListElement"),
    );
    map.insert(
        String::from("last"),
        String::from("Returns the last element of a list.\nList -> ListElement"),
    );
    map.insert(
        String::from("tail"),
        String::from("Returns all elements of a list except the first.\nList -> List"),
    );
    map.insert(
        String::from("get"),
        String::from(
            "Returns the element of a list specified by its index.\nList -> Number -> ListElement",
        ),
    );
    map.insert(
        String::from("map"),
        String::from("Applies a function to each element of a list and returns a new list.\nList -> Function -> List"),
    );
    map.insert(
        String::from("filter"),
        String::from("Filters a list based on a predicate function and returns a new (filtered) list.\nList -> Function -> List"),
    );
    map.insert(
        String::from("reverse"),
        String::from("Reverses a list.\nList -> List"),
    );

    // Functional Utilities
    map.insert(
        String::from("fold"),
        String::from("Reduces a list to a single value using a function.\nList -> Function -> InitialValue -> Value"),
    );

    // String Utilities
    map.insert(
        String::from("lowercase"),
        String::from("Returns the lowercase equivalent of the original String.\nString -> String"),
    );
    map.insert(
        String::from("uppercase"),
        String::from("Returns the uppercase equivalent of the original String.\nString -> String"),
    );
    map.insert(
        String::from("trim"),
        String::from(
            "Returns a String with leading and trailing white space removed.\nString -> String",
        ),
    );
    map.insert(
        String::from("parseNumber"),
        String::from("Converts a String into a number.\nString -> Number"),
    );
    map.insert(
        String::from("replaceString"),
        String::from("Replaces all matches of a pattern with a String.\nString -> Pattern(String) -> String(String) -> String"),
    );
    map.insert(
        String::from("replaceN"),
        String::from("Replaces the first N matches of a pattern with a String.\nString -> Replacement(String) -> Count(Number) -> String"),
    );
    map.insert(
        String::from("explode"),
        String::from("Converts a string to a list. Each of the characters in the string is given an index that starts from 0.\nString -> List"),
    );
    map
}
