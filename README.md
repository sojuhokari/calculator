# Simple Calculator

Here's the document with an introduction to Rust: [`example.rs`](example.rs)

This is the source code for a project to help people learn Rust.

## Goals!

These are the things that we'll hopefully learn by writing all of this

### Rust Language Features

* `while` loops
* `match` expressions
* expressions vs statements
* `loop` loops?
* functions!
    * returning expressions
* `enum`s
* error handling
* `String` vs `str`
* references!
* mutability
* heap vs stack

### General Programming Patterns

* functional programming
    * pure functions!!!
* recursion
* how to write an interpreter
    * this programming pattern is used in both programming language development AND to an extent in natural language processing (think Siri or Alexa)
    * lexers
    * recursive descent parsers
    * tree-walk interpreters

## Contents

[`cargo.toml`](cargo.toml) contains all the project metadata. We don't need to worry about that yet.

[`src/main.rs`](src/main.rs) contains the entrypoint to our program – the `main()` function. We use a `while` loop to keep the program going until the user enters `q` to quit, and we use Rust's `match` expression to handle errors from three functions: the lexer, parser, and interpreter.

[`src/lexer.rs`](src/lexer.rs) contains the lexer. This function takes an input string and tokenizes it, using a `peekable` `iterator` and some `while` and `match` magic, as well as an `enum`.

[`src/parser.rs`](src/parser.rs) contains the recursive descent parser. It uses recursive functions to parse a `vec` of tokens into an abstract syntax tree (AST), which is again made up of `enum`s.

[`src/interpreter.rs`](src/interpreter.rs) uses a tree-walk interpreter to walk the "leaves" of the AST to evaluate the expression, and returns the answer to the original math problem.