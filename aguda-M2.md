# Aguda Compiler - Milestone 2

## Compiler Techniques
### MSc in Computer Science & Engineering - Faculty of Sciences of the University of Lisbon
#### Summer Semester 2024/2025

> Ricardo Costa - 64371

---

Parser for the AGUDA programming language implemented in [Rust](https://www.rust-lang.org/) with [RUSTLR](https://chuckcscccl.github.io/rustlr_project/) as the parser generator.

### Running the Parser with Docker

```sh
docker build -t aguda-rs .
docker run aguda-rs cargo run
```

#### Running a Specific File

With the .agu file in the current directory, run:

```sh
docker run -v ./<filename>.agu aguda-rs cargo run <filename>.agu
```

#### Running the Tests

```sh
docker run aguda-rs cargo test
```

### Generating the Parser

To generate the parser from the grammar file, run:

```sh
cd src
rustlr rustlr.grammar
```

### Considerations

Initially, this phase of the project was implemented with a Logos as the lexer and LALRPOP as the (LR(1)) parser.
However, LALRPOP does not tolerate any conflicts and lacks the support for operator precedence and associativity.
Due do this, I couldn't resolve the shift-conflicts caused by the "dangling else".
So, I decided to switch to RUSTLR, which is a Yacc-like LALR(1) parser generator that supports operator precedence and associativity.
However, it lacks control over the parser, as it includes both the lexer and the parser.
RUSTLR does not differentiate between lexical and syntactic errors, and prints the error automatically, which prevented me to distinguish the two in the error messages.
Furthermore, RUSTLR comes with automatic AST generation, which was not desirable. Even so, I was able to convert the AST to my simplified version.
Additionally, since we wanted a custom identifier regex including `'` and in this tool custom token types override all others, I had to implement a custom regex for all alphanumeric tokens and place them in the right order to avoid the lexer recognizing keywords as identifiers. These represent the limitations of RUSTLR, which I was overcome, except for the differentiation of lexical from syntactic errors.