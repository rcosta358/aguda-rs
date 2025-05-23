# Aguda Compiler - Milestone 3

## Compiler Techniques
### MSc in Computer Science & Engineering - Faculty of Sciences of the University of Lisbon
#### Summer Semester 2024/2025

> Ricardo Costa - 64371

---

Parser for the AGUDA programming language implemented in [Rust](https://www.rust-lang.org/) with [Logos](https://logos.maciej.codes/) as the lexer generator and [LALRPOP](https://lalrpop.github.io/lalrpop/) as the parser generator.

In this phase, the semantic analysis was implemented, including the symbol table, the declaration checking and the type checking.

### Running with Docker

#### Prerequisites
- [Docker](https://www.docker.com/) installed and running
- [Docker Compose](https://docs.docker.com/compose/) installed

#### Build the Image

To build the Docker image, run:

```sh
docker-compose build --no-cache
```

#### Create and Access the Container

To create a container and spawn a shell inside it, run:

```sh
docker-compose run --rm aguda-rs bash
```

This will automatically clone the tests from the [aguda-testing](https://git.alunos.di.fc.ul.pt/tcomp000/aguda-testing) repository.
Also, it will also build and re-generate the parser with the [grammar file](./src/grammar.lalrpop).

#### Run the Compiler

Then, inside the container's shell, to run the compiler, run:

```sh
cargo run
```

#### Run the Tests
To run the tests, run:

```sh
cargo test -- --nocapture
```

#### Run a Specific File

To run a specific file, run:

```sh
cargo run -- --file path/to/file.agu
```

### Command Line Arguments

The compiler also accepts various command line arguments to customize its behavior:

| Option                          | Description                                | Default    |
|---------------------------------|--------------------------------------------|------------|
| `-f, --file <FILE>`             | Path to the source .agu file               | `main.agu` |
| `--max-errors <MAX_ERRORS>`     | Maximum number of errors to display        | `5`        |
| `--max-warnings <MAX_WARNINGS>` | Maximum number of warnings to display      | `5`        |
| `--suppress-errors`             | Suppress errors in the output              |            |
| `--suppress-warnings`           | Suppress warnings in the output            |            |
| `--suppress-hints`              | Suppress hints in the output               |            |
| `--ast`                         | Show the AST without running the program   |            |
| `-h, --help`                    | Print help                                 |            |
| `-V, --version`                 | Print version                              |            |

Example usage:

```sh
cargo run -- --file hello.agu --max-errors 10 --ast
```

### Implementation

#### Symbol Table

The symbol table is implemented as a stack of scopes, where each scope contains a hashmap with the variables declared in it.

The `declare` method is used to declare a new symbol in the current scope, returning true if the symbol was declared and false otherwise. The only case that the symbol is not declared is if we try to redeclare a symbol in the global scope, which is not allowed. It has a special case for the wildcard identifier `_`, which is ignored and not inserted into the table.

The `lookup` method is responsible for retrieving the type of a symbol by id, by looking for it in the current scope and all its parents, returning the first one found or `None` otherwise. This means that it allows for variable shadowing, since the each scope contains its own hashmap and when looking up a symbol it retrieves the most recently declared one. Again, if the symbol's identifier is `_`, it simply returns `None`.

Furthermore, the symbol table does not start empty. It is initialized with the functions `print` and `length`, which required the `Any` type to be introduced for their function signatures, since they can accept any type and any array type respectively.

The symbol table's implementation is in [`symbol_table.rs`](./src/semantic/symbol_table.rs).


#### Bidirectional Type Checking

After the declaration checking is done, we perform the type checking. Since the declaration checking still contains the global declarations and the `print` and `length` functions in the symbol table, it is then reused for the type checking, which will have a jump start in the analysis. 

The type checker is implemented with bidirectional type checking, with the mutual recursive methods `type_of` and `check_against`.
The `type_of` method synthesizes the type of an expression and the `check_against` analyzes it by checking it against the expected type.
The `check_against` method only has three cases: one for the `Any` type, another for an `Any[]` type and another for all other cases, which must match the expected type exactly.
Additionally, there is an extra method `check_equal`, which compares two types, in order to provide better error messages when two types that should match don't, for example in if expressions and in comparisons.

The type checker's implementation is in [`type_checker.rs`](./src/semantic/type_checker.rs).

#### Diagnostics

As an extra, diagnostics were also implemented. It includes warnings, hints and precise error messages, for a better user experience.

**Errors:**
- Lexical errors (e.g. invalid tokens, integer overflow, unterminated string, etc.)
- Syntax errors (e.g. unexpected token, unexpected end of file, etc.)
- Declaration errors (e.g. undeclared identifier, duplicate declaration, reserved identifier, function signature mismatch, etc.)
- Type errors (e.g. type mismatch, argument count mismatch, expression not callable/indexable, etc.)

**Warnings:**
- Unused variables
- Variable redefinition in the same scope (data overwriting)

**Hints:**
- Syntax error suggestions (e.g. missing semicolon, missing type annotation, confusing `=` with `==`, etc.)
- Similar identifiers when identifier not found (e.g. "did you mean `print`?")
- Unused variables (e.g. "if this is intentional, prefix it with an underscore: `_x`")
- Using an if without an else (e.g. "when using an if without an else, the then branch must be of type `Unit`")
