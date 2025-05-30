# Aguda Compiler - Milestone 4

## Compiler Techniques
### MSc in Computer Science & Engineering - Faculty of Sciences of the University of Lisbon
#### Summer Semester 2024/2025

> Ricardo Costa - 64371

---

Compiler for the AGUDA programming language implemented in [Rust](https://www.rust-lang.org/) with [Logos](https://logos.maciej.codes/) as the lexer generator and [LALRPOP](https://lalrpop.github.io/lalrpop/) as the parser generator.
Uses [Inkwell](https://github.com/TheDan64/inkwell) language binding to generate LLVM IR code.

In this phase, the code generation was implemented.

### Running with Docker

#### Prerequisites
- [Docker](https://www.docker.com/) installed and running
- [Docker Compose](https://docs.docker.com/compose/) installed

#### Build the Image

To build the Docker image, run:

```sh
docker-compose build
```

This uses a pre-built Docker image containing Rust, LLVM and Clang for faster builds.
Also, it will automatically:
- Clone the tests from the [aguda-testing](https://git.alunos.di.fc.ul.pt/tcomp000/aguda-testing) repository
- Re-generate the parser with the [grammar file](./src/grammar.lalrpop)
- Re-generate the [lib.c](./lib.c) into a `.ll` with `clang` to then be linked with the generated LLVM code

The C library includes:
- A print function for each type (`__print_int__`, `__print_bool__`, `__print_unit__`)
- A power function (`__pow__`) to compute the power of a number
- A division function (`__div__`) to check for division by zero at runtime

#### Create and Access the Container

To create a container with the image and spawn a shell inside it, run:

```sh
docker-compose run --rm aguda-rs bash
```

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

| Option                          | Description                              | Default    |
|---------------------------------|------------------------------------------|------------|
| `-f, --file <FILE>`             | Path to the source .agu file             | `main.agu` |
| `-o, --opt <OPT_LEVEL>`         | LLVM optimization level (0-3)            | `0`        |
| `--max-errors <MAX_ERRORS>`     | Maximum number of errors to display      | `5`        |
| `--max-warnings <MAX_WARNINGS>` | Maximum number of warnings to display    | `5`        |
| `--suppress-errors`             | Suppress errors in the output            |            |
| `--suppress-warnings`           | Suppress warnings in the output          |            |
| `--suppress-hints`              | Suppress hints in the output             |            |
| `--ast`                         | Show the AST without running the program |            |
| `-h, --help`                    | Print help                               |            |
| `-V, --version`                 | Print version                            |            |

Example usage:

```sh
cargo run -- -f main.agu -o 1 --max-errors 10
```

### Implementation

The code generation works by converting the AST into LLVM IR using the `CodeGen` struct that uses the Inkwell language binding.
This struct contains the following key components:
   - `context`: LLVM context for generating types and values
   - `module`: contains the generated LLVM IR
   - `builder`: for constructing LLVM instructions
   - `symbols`: the symbol table for declaring and looking up declarations

It firstly declares all function signatures first (for mutual recursion), then declares and generates all global variables and function bodies.
The end result is LLVM IR that is then executed using the `lli` interpreter.

The code generation implementation is in [`codegen.rs`](./src/codegen/codegen.rs).
This implementation neither supports top-level let expressions except for constants, nor arrays and strings.

#### Short-Circuit Boolean Expressions

The short-circuit boolean expressions (`&&`, `||` and `!`) were implemented to optimize boolean expressions.

For the `&&` and `||` operators, it is done as follows:

- The left operand is evaluated 
- Depending on the operation, proceeds as follows:
  - `&&`:
    - If the result of the left operand is `false`, then the right side is not evaluated and branches to the end of the boolean expression (short-circuit)
    - If the result of the left operand is `true`, then the right operand is evaluated
  - `||`:
    - If the result of the left operand is `true`, then the right side is not evaluated and branches to the end of the boolean expression (short-circuit)
    - If the result of the left operand is `false`, then the right operand is evaluated

For the `!` operator, it is done differently:

It works by counting the number of chained not operations in the expression.
- **Even Count**: the not operations cancel each other out, so no extra code is generated for the negation of the boolean expression, and just the innermost expression is generated
- **Odd Count**: a single not operation is generated to negate the innermost expression
