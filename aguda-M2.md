# Aguda Compiler - Milestone 2

## Compiler Techniques
### MSc in Computer Science & Engineering - Faculty of Sciences of the University of Lisbon
#### Summer Semester 2024/2025

> Ricardo Costa - 64371

---

Parser for the AGUDA programming language implemented in [Rust](https://www.rust-lang.org/) with [Logos](https://logos.maciej.codes/) as the lexer generator and [LALRPOP](https://lalrpop.github.io/lalrpop/) as the parser generator.

### Running the Parser with Docker

To build the image and run the container, run:

```sh
docker build -t aguda-rs .
docker run aguda-rs
```

This will also regenerate the parser with the [grammar file](./src/grammar.lalrpop).

### Program Arguments

| Option                          | Description                                               | Default    |
|---------------------------------|-----------------------------------------------------------|------------|
| `-f, --file <FILE>`             | Path to the source .agu file                              | `main.agu` |
| `--max-errors <MAX_ERRORS>`     | Maximum number of errors to display                       | `5`        |
| `--max-warnings <MAX_WARNINGS>` | Maximum number of warnings to display                     | `5`        |
| `--suppress-errors`             | Suppress errors in the output                             |            |
| `--suppress-warnings`           | Suppress warnings in the output                           |            |
| `--suppress-hints`              | Suppress hints in the output                              |            |
| `--suppress-ast`                | Suppress the textual representation of the AST in output  |            |
| `--suppress-all`                | Suppress all output                                       |            |
| `-h, --help`                    | Print help                                                |            |
| `-V, --version`                 | Print version                                             |            |

Example usage:

```sh
docker run aguda-rs cargo run -- --file hello.agu --max-errors 10 --suppress-ast
```

#### Running the Tests

To run the test pool, run:

```sh
docker run aguda-rs cargo test -- --nocapture
```

### Test Results

Since the language won't support higher-order functions, my parser only considers types to be basic types (`Int`, `Bool`, `String`, `Unit`) and arrays of these. However, there are some tests that assume that the types can be function types, namely in the function signatures.
