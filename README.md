# Beavieeer-Lang

Welcome to the **Beavieeer-Lang Project**, a compiler written in the [Rust programming language](https://www.rust-lang.org/) for a C-like toy programming language. This project takes concepts from the book [*Writing a Compiler in Go*](https://compilerbook.com/) and implements them using Rust, showcasing its low-level control and safety.

The toy language has a C-like syntax, supporting basic constructs like variable declarations, arithmetic expressions, conditionals, and function calls. The goal is to build a working compiler that translates this language into bytecode, which can then be executed on a custom virtual machine (VM).

---

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

---

## Introduction

The goal of this project is to provide an educational and practical implementation of a compiler written in Zig. Inspired by *Writing a Compiler in Go*, this project walks through the design and implementation of a C-like programming language, complete with a lexer, parser, abstract syntax tree (AST), and virtual machine (VM).

Whether you're learning about compiler design or exploring Zig's capabilities, this repository offers a hands-on journey.

---

## Features

- **Lexer**: Tokenizes input source code into meaningful symbols.
- **Parser**: Converts tokens into an Abstract Syntax Tree (AST).
- **AST Evaluation**: Processes the AST to generate executable bytecode.
- **Virtual Machine (VM)**: Executes bytecode instructions.
- **Built-in Functions**: Supports basic built-in functionality.
- **Error Handling**: Provides detailed syntax and runtime error messages.

---

## Getting Started

### Prerequisites

- [Zig](https://ziglang.org/download/): Ensure you have Zig installed (v0.10.0 or higher recommended).
- A basic understanding of compiler design and Zig programming will be helpful.

### Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/your-username/Beavieeer.git
   cd Beavieeer 
   ```
2. Build the project:
   ```bash
   cargo build
   ```
3. Run the tests:
   ```bash
   cargo build 
   ```

---

## Usage

To compile and execute code written in Beavieeer language, follow these steps:

1. Write your source code in a `.be` file. Example:
   ```Rust
   let x = 5;
   let y = 10;
   let result = x + y;
   print(result);
   ```

2. Run the compiler:
   ```bash
   cargo run --path/to/source.be
   ```

3. The output of the program will be displayed in the terminal.

---

## Project Structure

```
├── src/
│   ├── lexer.rs          # Tokenizer implementation
│   ├── parser.rs         # Parser implementation
│   ├── ast.rs            # Abstract Syntax Tree structures
│   ├── codegen.rs        # Bytecode generator
│   ├── vm.rs             # Virtual Machine
│   └── main.rs           # Entry point for the compiler
└── README.md               # Project documentation
```

---

## Contributing

Contributions are welcome! If you would like to improve this project, please:

1. Fork the repository.
2. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/my-feature
   ```
3. Commit your changes:
   ```bash
   git commit -m "Add my feature"
   ```
4. Push your branch:
   ```bash
   git push origin feature/my-feature
   ```
5. Open a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
