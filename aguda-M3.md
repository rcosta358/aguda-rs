# Aguda Compiler - Milestone 3

## Compiler Techniques
### MSc in Computer Science & Engineering - Faculty of Sciences of the University of Lisbon
#### Summer Semester 2024/2025

> Ricardo Costa - 64371

---

Parser for the AGUDA programming language implemented in [Rust](https://www.rust-lang.org/) with [Logos](https://logos.maciej.codes/) as the lexer generator and [LALRPOP](https://lalrpop.github.io/lalrpop/) as the parser generator.

In this phase, the semantic analysis was implemented, including the symbol table, the declaration checking and the type checking.

### Running with Docker

To build the image and run the container, run:

```sh
docker build -t aguda-rs .
docker run aguda-rs
```

This will also regenerate the parser with the [grammar file](./src/grammar.lalrpop).

### Command Line Arguments

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

To change the directory where to look for the tests, modify the following line in [`parser_tests.rs`](./tests/parser_tests.rs):

```rust
let base_dir = Path::new("./tests/"); // replace with the desired directory
```

##### Test Results

Since the language won't support higher-order functions, my parser only considers types to be basic types (`Int`, `Bool`, `String`, `Unit`) and arrays of these. However, there are some tests that assume that the types can be function types, namely in the function signatures.

### Implementation

#### Symbol Table

The symbol table is implemented as a stack of scopes, where each scope contains a hashmap with the variables declared in it. 
When declaring a new symbol, the symbol table checks if it already exists in the current scope - if so, it returns an error, otherwise it inserts the symbol into the current scope's hashmap. This means that identifiers can be shadowed if declared in different scopes. However, if the symbol's identifier is `_`, it is simply ignored and not inserted into the table, since wildcards can be duplicated and not looked up.

When looking up a symbol, the symbol table checks for it in the current and all parent scopes, returning the first one found (closest), with the type of the symbol and `None` otherwise. Again, if the symbol's identifier is `_`, it simply returns `None`.

Furthermore, the symbol table does not start empty. It is initialized with the functions `print` and `length`, which required the `Any` type to be introduced for their function signatures, since they can accept any type and any array type respectively.

The symbol table's implementation is in [`symbol_table.rs`](./src/semantic/symbol_table.rs).


#### Bidirectional Type Checking

After the declaration checking is done, if there are no errors, since the declaration checking still contains the global declarations and the `print` and `length` functions in the symbol table, it is then reused for the type checking, which will have a jump start in the analysis. 

The type checker is implemented with bidirectional type checking, with `type_of` and `check_against`.
The `type_of` function synthesizes the type of an expression and the `check_against` analyzes it by checking it against the expected type.
The `check_against` function only has three cases: one for the `Any` type, another for an `Any[]` type and another for all other cases, which must match the expected type exactly.
There is an extra function `check_equal`, which compares two types, in order to provide better error messages when two types that should match don't, for example in if expressions and in comparisons.

The type checker's implementation is in [`type_checker.rs`](./src/semantic/type_checker.rs).