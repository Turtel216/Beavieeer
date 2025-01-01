# Beavieeer Language

<p align="center">
  <img src="imgs/Beavieeer-Mascot.jpg" alt="Beavieeer Logo" width="150" style="border-radius: 50%;">
</p>

## About

**Beavieeer** is a simple, C-like interpreted toy programming language written in Rust. This repository contains the implementation of the interpreter for Beavieeer. The language is designed to be easy to learn, with basic control structures, data types, and functional programming features. It supports both running scripts from `.be` files and an interactive REPL (Read-Eval-Print Loop).

---

## Features

### Core Language Features:
- **Control Structures**: `if` statements for conditional logic.
- **Data Types**: 
  - **Primitive types**: `Integer`, `String`, and `Boolean`.
  - **Complex types**: `Array` and `Hash`.
- **Anonymous Functions**: Functions that can be defined without a name and passed around as values.
  
### Standard Library:
- **I/O**: 
  - `print` - Prints a value to the console.
- **Array Operations**: 
  - `first` - Returns the first element of an array.
  - `last` - Returns the last element of an array.
  - `rest` - Returns all elements of an array except the first.
  - `map` - Applies a function to each element of an array and returns a new array.
  - `filter` - Filters an array based on a predicate function.
  - `sort` - Sorts an array.
- **Functional Utilities**: 
  - `fold` - Reduces an array to a single value using a function.

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
```c
let x = [1, 2, 3, 4, 5];
let doubled = map(x, fun(n) { n * 2 });
print(doubled);
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
```c
let x = 42;
let name = "Beavieeer";
let isAwesome = true;
```

### If Statements
```c
if (x > 10) {
    print("x is greater than 10");
} else {
    print("x is less than or equal to 10");
}
```

### Arrays
```c
let nums = [1, 2, 3, 4];
print(first(nums)); // 1
print(last(nums));  // 4
print(rest(nums));  // [2, 3, 4]
```

### Anonymous Functions
```c
let square = fun(n) { n * n };
print(map([1, 2, 3, 4], square)); // [1, 4, 9, 16]
```

### Hashes
```c
let person = {"name": "Alice", "age": 30};
print(person["name"]); // Alice
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
For any questions or suggestions, feel free to open an issue or contact the maintainer at `papakonstantinou.dm@gmail.com`.
