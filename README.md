# AGUDA Compiler in Rust 🏝️🦀

## Introduction

Compiler for the AGUDA programming language implemented in Rust for the Compilation Techniques course.
AGUDA is an imperative language where programs consist solely of expressions.
Each program is a sequence of declarations, introduced by the `let` keyword.

## Syntax

### Expressions

- **Variable**: `id`
- **Literals**: `...`, `-1`, `0`, `1`, `...`, `true`, `false`, `null`, `"string"`
- **Binary operators**: `;`, `+`, `-`, `*`, `/`, `%`, `^`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `!`, `||`, `&&`
- **Unary operators**: `-`, `!`
- **Function call**: `id(exp1,...,expn)` (n >= 1)
- **Assignment**: `set lhs = exp`
- **Variable declarations**: `let id : type = exp`
- **Conditionals**: `if exp1 then exp2 else exp3`, `if exp1 then exp2`
- **While loop**: `while exp1 do exp2`
- **Array creation**: `new type [ exp1 | exp2 ]`
- **Array access**: `exp1[exp2]`
- **Parenthetical expression**: `(exp)`

### Declarations

- **Variables**: `let id : type = exp`
- **Functions**: `let id (id1, ..., idn) : type = exp` (n >= 1)

### Types

- **Basic**: `Int`, `Bool`, `Unit`, `String`
- **Arrays**: `type []`
- **Functions**: `type -> type` or `(type1, ..., type) -> type` (n >= 1)


## Example

Here's a simple AGUDA program that creates a 2x2 identity matrix and prints it:

```aguda
let printMatrix (a) : Int[][] -> Unit =
    let i : Int = 0;
    while i < length(a) do (
        let j : Int = 0;
        while j < length(a[0]) do (
            print(a[i][j]); print(" ");
            set j = j + 1
        );
        print("\n");
        set i = i + 1
    )

let main : Unit =
    let a : Int[][] = new Int[][2 | new Int[2 | 0]];
    set a[0][0] = 1;
    set a[1][1] = 1;
    printMatrix(a)
```