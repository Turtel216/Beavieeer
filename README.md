# Beavieeer Language

## Table of Contents
- [About](#about)
- [Features](#features)
  - [Core Language Features](#core-language-features)
  - [Standard Library](#standard-library)
  - [Execution Modes](#execution-modes)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
  - [Script Mode](#script-mode)
  - [Interactive REPL](#interactive-repl)
- [Language Syntax](#language-syntax)
  - [Variables](#variables)
  - [If Statements](#if-statements)
  - [If Expressions](#if-expressions)
  - [List](#list)
  - [Anonymous Functions](#anonymous-functions)
  - [Hashes](#hashes)
  - [Strings](#strings)
- [Future Features](#future-features)
- [Benchmarking Performance](#benchmarking-performance)
  - [Benchmark Setup](#benchmark-setup)
  - [Benchmark Results](#benchmark-results)
  - [Running the Benchmark Yourself](#running-the-benchmark-yourself)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

---

## About

**Beavieeer** is a simple, C-like interpreted toy programming language written in Rust. This repository contains the implementation of the interpreter for Beavieeer. The language is designed to be easy to learn, with basic control structures, data types, and functional programming features. It supports both running scripts from `.be` files and an interactive REPL (Read-Eval-Print Loop).

---

## Features

### Core Language Features:
- **Control Structures**: `if` statements for conditional logic.
- **Data Types**: 
  - **Primitive types**: `Integer`, `String`, and `Boolean`.
  - **Complex types**: `List` and `Hash`.
- **Anonymous Functions**: Functions that can be defined without a name and passed around as values.
- **Higher-order function**: A function that takes one or more functions as arguments
  
### Standard Library:

- **I/O**: 
  - `print` - Prints a value to the console.
  - `read` - Read a value from the console.
  - `readFile` - Read the contents of a file.
  - `writeFile` - Writes to a file the given content `String`. Creates a file if it does not exist.
- **List Operations**: 
  - `len` - Returns the length of a given list.
  - `first` - Returns the first element of a list.
  - `last` - Returns the last element of a list.
  - `tail` - Returns all elements of a list except the first.
  - `get` - Returns the element of a list specified by its index.
  - `map` - Applies a function to each element of a list and returns a new list.
  - `filter` - Filters a list based on a predicate function and returns a new (filtered)list.
  - `reverse` - Reverses a list.
- **Functional Utilities**: 
  - `fold` - Reduces a list to a single value using a function.
- **String Utilities**:
  - `lowercase` - Returns the lowercase equivalent of the original String
  - `uppercase` - Returns the uppercase equivalent of the original String
  - `trim` - Returns a String with leading and trailing white space removed
  - `parseNumber` - Converts a String into a number
  - `replaceString` - Replaces all matches of a pattern with a String
  - `replaceN` - Replaces the first N matches of a pattern with a String
  - `explode` - Converts a string to a list. Each of the characters in the string is given an index that starts from 0
  - **More to be added**

### Execution Modes:
- **Script Mode**: Run Beavieeer programs from a file with the `.be` extension.
- **Interactive REPL**: Explore and test Beavieeer code interactively. Exit the REPL by typing `:q`.

---

## Getting Started

### Prerequisites
- **Rust**: Ensure you have Rust installed. If not, install it using [Rust's official guide](https://www.rust-lang.org/tools/install).

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Turtel216/Beavieeer.git
   cd Beavieeer
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the interpreter:
   ```bash
   ./target/release/beavieeer
   ```

---

## Usage

### Script Mode
To execute a Beavieeer script, create a file with the `.be` extension and run:
```bash
./beavieeer path/to/script.be
```

Example script:
```kotlin
let fibonacci = fun(x) {
  if (x == 0) {
    0;
  } else {
    if (x == 1) {
      1;
    } else {
      fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
};

let result = fibonacci(10);
print(result);
```

### Interactive REPL
Start the REPL:
```bash
./beavieeer
```
You can now write and execute Beavieeer code interactively. Exit the REPL by typing `:q`.

---

## Language Syntax

### Variables
```kotlin
let x = 42;
let name = "Beavieeer";
let isAwesome = true;
```

### If Statements
```kotlin
if (x > 10) {
    print("x is greater than 10");
} else {
    print("x is less than or equal to 10");
}
```

### If Expressions
```kotlin
let x = 1;
let y = if (x == 2) { 2 } else { 1 };
print(y); // 1
```

### List
```kotlin
let nums = [1, 2, 3, 4];
print(nums[1]); // 2
print(first(nums)); // 1
print(last(nums));  // 4
print(tail(nums));  // [2, 3, 4]
```

Beavieeer also supports adding lists

```kotlin
let lstOne = [1, 2, 3, 4];
let lstTwo = [5, 6, 7, 8];
let lstCombined = lstOne + lstTwo // [1, 2 ,3 ,4 ,5 ,6 ,7 ,8]
```

### Anonymous Functions
```kotlin
let square = fun(n) { n * n };
print(map([1, 2, 3, 4], square)); // [1, 4, 9, 16]
```

### Hashes
```kotlin
let person = {"name": "Alice", "age": 30};
print(person["name"]); // Alice
```

### Strings
```kotlin
let hello = "Hello, ";
let world = "World!";
let contact = hello + world;
print(contact); // Hello, World!

let upper = uppercase(contact);
print(upper); // HELLO, WORLD!

let lower = lowercase(upper);
print(lower); // hello, world!

let replaced = replaceString(lower, "world", "beavieeer");
print(replaced); // hello, beavieeer!

let hey = "Hey Hey Hey";
let bye = replaceN(spam, "Hey", "Bye", 2);
print(bye); // Bye Bye Hey
```

---

## Future Feautes

- [ ] `import` buildin for importing other Beavieeer files.
- [ ] **http utility** functions.
- [ ] **REPL** improvements.

---

Here’s a **GitHub README section** to document your benchmarks in a structured and professional way.  

---

## Benchmarking Performance

We benchmarked our interpreter against Python using a **Fibonacci sequence calculation** to measure execution speed. The tests were conducted using [`hyperfine`](https://github.com/sharkdp/hyperfine), a reliable benchmarking tool.

### 📌 **Benchmark Setup**
1. **Comparison**: Our interpreter vs. Python 3.
2. **Tool Used**: [`hyperfine`](https://github.com/sharkdp/hyperfine) (for accurate timing).
3. **Command Used**:
    ```sh
    hyperfine "python3 ./benchmarks/fibonacci.py" "./target/release/beavieeer ./benchmarks/fibonacci.be"
    ```

---

### Benchmark Results
| Language | Execution Time (mean ± σ) | Speed Factor |
|----------|-------------------------|-------------|
| **Python 3** | 115.8 ms ± 0.7 ms | **1x (Baseline)** |
| **Beavieer 1.2** | 3.020 s ± 0.096 s | **~26.07x slower** |
| **Beavieer 1.3** | 2.841 s ± 0.086 s | **~24.38x slower** |

---

### Running the Benchmark Yourself
#### Install Hyperfine (if not installed)
```sh
# On Linux/macOS
sudo apt install hyperfine   # Debian/Ubuntu
brew install hyperfine       # macOS (Homebrew)

# On Windows (via Cargo)
cargo install hyperfine
```

#### Run the Benchmark 
```sh
hyperfine "python3 ./benchmarks/fibonacci.py" "./target/release/beavieeer ./benchmarks/fibonacci.be"
```

---

## Contributing
Contributions are welcome! Please open an issue or submit a pull request if you have ideas for improvements.

### Steps to Contribute
1. Fork the repository.
2. Create a new branch for your feature/fix:
   ```bash
   git checkout -b feature-name
   ```
3. Commit your changes and push to your fork.
4. Open a pull request to the `main` branch.

---

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contact
For any questions or suggestions, feel free to open an issue or cOntact the maintainer at `papakonstantinou.dm@gmail.com`.
