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
docker-compose build --no-cache
```

**Warning:** this build takes a long time because it needs to download LLVM.

This will automatically clone the tests from the [aguda-testing](https://git.alunos.di.fc.ul.pt/tcomp000/aguda-testing) repository.
Also, it will build and re-generate the parser with the [grammar file](./src/grammar.lalrpop) as well as the C library into a `.ll` with `clang` to then be linked with the generated LLVM code.
The C library includes the definitions of the various print functions and the pow and div functions. Each type has its corresponding print function, the pow is needed because LLVM does not have a power operation and the div is needed to check for division by zero.

#### Create and Access the Container

To create a container and spawn a shell inside it, run:

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
cargo run -- -f hello.agu -o 1 --max-errors 10
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

The short-circuit boolean expressions (`&&` and `||`) were implemented using a custom function that:

1. Evaluates the left operand
2. Depending on the operation, proceeds as follows:
  - `&&`:
    - If the result of the left operand is `false`, then the right side is not evaluated and branches to the end of the boolean expression (short-circuit)
    - If the result of the left operand is `true`, then the right operand is evaluated
  - `||`:
    - If the result of the left operand is `true`, then the right side is not evaluated and branches to the end of the boolean expression (short-circuit)
    - If the result of the left operand is `false`, then the right operand is evaluated
3. In the end, a phi node is used to select the result