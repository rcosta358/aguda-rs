# Aguda Compiler - Milestone 2

## Compiler Techniques
### MSc in Computer Science & Engineering - Faculty of Sciences of the University of Lisbon
#### Summer Semester 2024/2025

> Ricardo Costa - 64371

---

Parser for the AGUDA programming language implemented in [Rust](https://www.rust-lang.org/) with [RUSTLR](https://chuckcscccl.github.io/rustlr_project/) as the lexer and parser generator.

### Running the Parser with Docker

To build the image and run the container, run:

```sh
docker build -t aguda-rs .
docker run aguda-rs
```

This will also regenerate the parser with the [grammar file](./src/rustlr.grammar).

#### Running a Specific File

To run a specific file, place the file in the root directory, rebuild the image and run:

```sh
docker run aguda-rs cargo run <filename>.agu
```

#### Running the Tests

To run the test pool, run:

```sh
docker run aguda-rs cargo test -- --nocapture
```

### Test Results

Since the language won't support higher-order functions, my parser only considers types to be basic types (`Int`, `Bool`, `String`, `Unit`) and arrays of these. However, there are some tests that assume that the types can be function types, namely in the function signatures.

The rest of the tests fail because the test pool still has some tests with syntax errors.

Additionally, in two tests, there were symbols that were not UTF-8, namely `§` and `á`, which caused the execution of the tests to panic and exit. This happens because Rust strings are UTF-8 encoded. To fix this, I replaced these symbols with `$` and `a` respectively, to be able to run the tests normally. 

### Challenges

Initially, this phase of the project was implemented with Logos as the lexer and LALRPOP as the parser (LR(1)).
However, LALRPOP does not tolerate any conflicts and lacks the support for operator precedence and associativity.
Due to this, I couldn't resolve the shift-reduce conflicts caused by the "dangling else", which forced me to only have matched if-else statements inside expressions, which was not the intended behavior of the language.

So, I decided to switch to RUSTLR, which is a Yacc-like LALR(1) parser generator that supports operator precedence and associativity.
RUSTLR comes with a lexer generator, a parser generator and automatic AST generation. Because of this, I had to extend the lexer to distinguish between lexical and syntactic errors. Also, I was able to convert the generated AST to a more simplified version of it, to make it easier to implement its textual representation as well as for the future phases of the compiler.

These briefly describe the challenges I faced, which I was able to overcome.