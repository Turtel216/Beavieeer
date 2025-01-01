// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

use beavieeer::repl;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    repl::start(&mut stdin_lock, &mut stdout_lock);
}
