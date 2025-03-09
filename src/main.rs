// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

use beavieeer::repl;
use std::env;
use std::fs;
use std::io::stdout;

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdout = stdout();
    let mut stdout_lock = stdout.lock();

    if args.len() == 1 {
        repl::start(&mut stdout_lock);
    } else if args.len() == 2 {
        let contents =
            fs::read_to_string(args[1].clone()).expect("Should have been able to read the file");
        repl::run_file(&contents);
    } else {
        println!("Invalid arguments");
    }
}
