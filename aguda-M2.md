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

This will also regenerate the parser with the grammar file.

#### Running a Specific File

To run a specific file, just place the file in the root directory and rebuild the image and run:

```sh
docker run aguda-rs cargo run <filename>.agu
```

#### Running the Tests

To run the test pool, run:

```sh
docker run aguda-rs cargo test -- --nocapture
```

### Test Results

Some tests fail because the test pool still has some tests with errors, namely regarding the syntax of the language.

### Challenges

Initially, this phase of the project was implemented with Logos as the lexer and LALRPOP as the parser (LR(1)).
However, LALRPOP does not tolerate any conflicts and lacks the support for operator precedence and associativity.
Due to this, I couldn't resolve the shift-reduce conflicts caused by the "dangling else", which forced me to only have matched if-else statements inside expressions, which was not the intended behavior of the language.

So, I decided to switch to RUSTLR, which is a Yacc-like LALR(1) parser generator that supports operator precedence and associativity.
RUSTLR comes with a lexer generator, a parser generator and automatic AST generation. Because of this, I had to extend the lexer to distinguish between lexical and syntactic errors. Also, I was able to convert the generated AST to a more simplified version of it, to make it easier to implement its textual representation as well as for the future phases of the compiler.

These briefly describe the challenges I faced, which I was able to overcome.