# Aguda Compiler - Milestone 2

## Compiler Techniques
### MSc in Computer Science & Engineering - Faculty of Sciences of the University of Lisbon
#### Summer Semester 2024/2025

> Ricardo Costa - 64371

---

Parser for the AGUDA programming language implemented in [Rust](https://www.rust-lang.org/) with [RUSTLR](https://chuckcscccl.github.io/rustlr_project/) as the lexer and parser generator.

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
docker run aguda-rs cargo test -- --nocapture
```

### Generating the Parser

To generate the parser from the grammar file, run:

```sh
cd src
rustlr rustlr.grammar
```

### Considerations

Initially, this phase of the project was implemented with Logos as the lexer and LALRPOP as the parser (LR(1)).
However, LALRPOP does not tolerate any conflicts and lacks the support for operator precedence and associativity.
Due to this, I couldn't resolve the shift-conflicts caused by the "dangling else".

So, I decided to switch to RUSTLR, which is a Yacc-like LALR(1) parser generator that supports operator precedence and associativity. However, it lacks control over the parser, includes both the parser and lexer, does not differentiate between lexical and syntactic errors and internally prints errors automatically.
Furthermore, RUSTLR comes with automatic AST generation, which was not what I was really looking for. Nevertheless, I was able to convert the generated AST to my simplified version of it, to make it easier to implement the textual representation of the AST as well as for the future phases of the compiler.

These represent the limitations of RUSTLR, which I was able to overcome, except for the differentiation between lexical and syntactic errors.