# **rust-from-zero**

A comprehensive, hands-on repository designed to guide learners from the fundamentals of Rust to more advanced concepts. Each folder contains a standalone Cargo project demonstrating a specific topic, complete with source code and explanations. Use this repository to experiment, study, and build confidence in writing idiomatic Rust.

---

## Table of Contents

1. [Introduction](#introduction)
2. [Prerequisites](#prerequisites)
3. [Getting Started](#getting-started)
4. [Project Structure](#project-structure)
5. [Usage](#usage)
6. [Contributing](#contributing)
7. [License](#license)

---

## Introduction

Rust is a systems‐level programming language with a focus on safety, concurrency, and performance. This repository—**rust-from-zero**—is organized into incremental examples, starting from “Hello, Rust!” all the way through ownership, borrowing, iterators, and generics. Each folder is a separate Cargo workspace that you can compile and run independently. The goal is to gradually build your Rust knowledge by walking through bite-sized, well‐documented code samples.

---

## Prerequisites

* **Rust toolchain** (stable): Install from [https://rustup.rs](https://rustup.rs)
* **Cargo** (comes bundled with Rust): Used to build and manage Rust projects.
* Basic familiarity with the command line (terminal).

---

## Getting Started

1. **Clone the repository**

   ```bash
   git clone https://github.com/<your-username>/rust-from-zero.git
   cd rust-from-zero
   ```
2. **Navigate to any example folder**
   Each folder is a standalone Cargo project. For example:

   ```bash
   cd r0-hellorust
   ```
3. **Build and run**

   ```bash
   cargo run
   ```

   You should see output corresponding to that example’s topic.

---

## Project Structure

Below is a high‐level overview of each folder and the Rust concepts it covers:

### r0-hellorust

> **Hello, Rust!**
> A minimal “Hello, Rust!” example demonstrating how to initialize a new Cargo project and print text to the console.

### r0-helloworld

> **Hello, World!**
> Similar to `r0-hellorust`, reinforcing how to structure `main.rs` and use `println!`.

---

### r1-variables

> **Variables & Mutability**
> Shows how to declare mutable and immutable variables, and how attempting to reassign an immutable variable results in a compile‐time error.

### r1-arrays

> **Arrays**
> Demonstrates fixed‐size arrays, indexing, and basic iteration over array elements.

### r1-even\_num

> **`is_even` Function**
> Implements a simple function to determine if a given integer is even, illustrating function definitions and control flow.

### r1-fibinocci

> **Fibonacci Series**
> Generates and prints the Fibonacci sequence, covering loops and numeric operations.

### r1-strings

> **Strings**
> Explores Rust’s `String` and `&str` types, showing how to concatenate, slice, and manipulate text.

### r1-tuples

> **Tuples**
> Introduces tuple creation, indexing, and pattern matching (destructuring) of tuples.

### r1-variables

> **Immutable Variable Assignment**
> Walks through examples of removing or changing immutable variable assignments to illustrate compiler errors and best practices.

---

### r2-enums-used-in-fns

> **Enums in Functions**
> Uses custom enums within function parameters and match expressions to showcase how to model state.

### r2-enums

> **Enum Basics**
> Defines and matches on simple enums, demonstrating idiomatic enum usage in Rust.

### r2-option-enums

> **`Option` Enum**
> Shows how to use `Option<T>` for nullable types, with examples of `Some` and `None`, and pattern matching.

### r2-result-enums

> **`Result` Enum**
> Covers error handling using `Result<T, E>`, including `Ok`, `Err`, and the `?` operator.

### r2-struct-with-fn

> **Structs with Methods**
> Defines a `struct` along with `impl` blocks to add methods, demonstrating how to encapsulate behavior.

### r2-structs

> **Struct Basics**
> Introduces basic `struct` definitions, field access, and creation of instances.

---

### r3-ownership

> **Ownership**
> Illustrates Rust’s ownership model: move semantics, the borrow checker, and variable scope.

### r3-borrowing

> **Borrowing**
> Demonstrates references (`&T`) and mutable references (`&mut T`), including examples of borrowing rules in action.

### r3-memory-management

> **Memory Management: Stack vs. Heap**
> Explains how data is allocated on the stack versus the heap, with examples showing when data is moved or cloned.

### r3-packages

> **External Packages**
> Shows how to add and use external crates (e.g., the `rand` crate) via `Cargo.toml`, including a simple random number generator example.

---

### r4-vectors

> **Vectors**
> Introduces dynamically‐sized arrays (`Vec<T>`), illustrating push, pop, and iteration.

### r4-array-slices

> **Array Slices**
> Demonstrates how to borrow parts of arrays or vectors using slices (`&[T]`) and shows common use cases.

### r4-string-slices

> **String Slices**
> Covers slicing `String` and `&str` types, highlighting how to work with Unicode boundaries.

### r4-hashmaps

> **HashMaps**
> Shows how to create, insert, access, and iterate over `HashMap<K, V>`.

### r4-iterators

> **Iterators**
> Explains the `IntoIterator` trait and how to call `.iter()`, `.into_iter()`, and `.iter_mut()` on collections.

### r4-iterator-adaptors

> **Iterator Adapters (map, filter)**
> Demonstrates functional‐style transformations using `.map()` and `.filter()` over iterators.

### r4-iterators-in-hashmaps

> **Iterating HashMaps**
> Shows how to iterate over key‐value pairs by reference, destructuring tuples in `for (key, value) in &map`.

### r4-consumer-adaptors

> **Iterator Consumers**
> Explores methods like `.collect()`, `.sum()`, and other iterator consumers that terminate the chain.

### r4-iterators-assignment

> **Assignment: Filter & Double**
> A practice exercise: filter all odd numbers from a vector, then double each remaining element using iterator chains.

### r4-vectors-to-hashmaps

> **Vector→HashMap Conversion**
> Converts a `Vec<(K, V)>` into `HashMap<K, V>` using `collect()` and illustrates when this pattern is useful.

---
### r5-pattern-matching
> Pattern Matching with `match` and `if let`
> Pattern matching is a powerful control-flow construct in Rust. This example introduces match expressions with primitives, enums, tuples, Option, and Result. It also demonstrates if let for cleaner code in some situations.

### r5-generics

> **Generics**
> Introduces generic functions, structs, and enums—showing how to write type-agnostic code and enforce trait bounds (e.g., `T: Display`).


### r5-traits
> **Traits and Trait Implementation**
>This example introduces traits—Rust's way of defining shared behavior—and how to implement them for custom types.

### r5-traits
> **Traits and Trait Bounds with impl Trait Syntax**
> This example demonstrates how to define and implement traits in Rust, and how to create functions that accept trait-bound parameters using the impl Trait syntax.



---


### r6-lifetimes
> **Understanding Lifetimes and Borrow Checker in Rust**
> This example demonstrates how Rust uses lifetimes to ensure memory safety by preventing dangling references. It introduces explicit lifetime annotations using 'a and explains how Rust determines which data lives long enough to be safely referenced.

### r6-structs-with-lifetimes
> **Using Lifetimes in Structs: Why Borrowed Data Must Outlive the Struct**
> This example introduces how to use lifetimes with struct fields that borrow data, and why lifetimes must be carefully managed to avoid dangling references.

### r6-threads
> **Advanced Multithreading in Rust with Multiple Threads and  Synchronization**
>Threads may interleave execution based on sleep durations and CPU scheduling.
> join() ensures deterministic cleanup by waiting for threads to finish before the program exits.

### r6-move-closures-with-threads
> **Using move Closures to Transfer Ownership into Threads**
>This example dives deeper into how ownership works with threads in Rust. Since threads may outlive the main function's stack, Rust enforces ownership rules strictly to avoid data races and invalid references

### r6-message-passing
>**Message Passing with Threads in Rust using mpsc::channel**
>This example demonstrates safe communication between threads using the message passing model in Rust. The mpsc module provides multi-producer, single-consumer channels, ideal for decoupled thread communication

### r6-future-trait : 
> **Understanding Async and Futures in Rust**
> This example introduces Rust's async/await model using the futures crate.

##### r6-async-await
> It explains how `async` functions work in Rust and how to `await` their results using the Tokio runtime's procedural macro.
> Demonstrates usage of async functions with `tokio::main` macro
> Shows how to use `tokio::time::sleep` for non-blocking delay

---

## Usage

For each example folder:

1. Open a terminal and navigate into the folder, e.g.:

   ```bash
   cd r1-arrays
   ```
2. Compile and run with Cargo:

   ```bash
   cargo run
   ```
3. Read the inline comments in `src/main.rs` (or `lib.rs`) for explanations of each concept.

Feel free to modify, extend, or experiment with the code in each folder to deepen your understanding.

---

## Contributing

Contributions are welcome! If you notice typos, want to suggest improvements, or add new examples:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-topic`).
3. Make your changes, ensuring each new example follows the existing naming conventions (e.g., `rX-your_topic`).
4. Submit a pull request, describing the new example or fix.

Please keep each example focused on a single concept, and include comments to explain key points.

---

## License

This repository is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

*Last updated: June 2025*
