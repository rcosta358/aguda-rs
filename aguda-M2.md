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

| Option                     | Description                     | Default     |
|----------------------------|---------------------------------|-------------|
| `-f`, `--file <...>`       | Path to the source `.agu` file  | `main.agu`  |
| `-m`, `--max-errors <...>` | Max number of errors to display | `10`        |
| `--no-print-ast`           | Skip printing the AST to stdout |             |
| `-h`, `--help`             | Show help message               |             |
| `-V`, `--version`          | Show version information        |             |

Example usage:

```sh
docker run aguda-rs cargo run -- --file hello.agu --max-errors 5 --no-print-ast
```

#### Running the Tests

To run the test pool, run:

```sh
docker run aguda-rs cargo test -- --nocapture
```

### Test Results

Since the language won't support higher-order functions, my parser only considers types to be basic types (`Int`, `Bool`, `String`, `Unit`) and arrays of these. However, there are some tests that assume that the types can be function types, namely in the function signatures.

Additionally, in two tests, there were symbols that were not UTF-8, namely `§` and `á`, which caused the execution of the tests to panic and exit. This happens because Rust strings are UTF-8 encoded. To fix this, I replaced these symbols with `$` and `a` respectively, to be able to run the tests normally. 
