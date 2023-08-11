# pico-typechecker

This is a smol project I'm messing around with,
I've been thinking about how typecheckers work and what better way to learn about typecheckers than to write one.
The code for the typechecker is in [typechecker.rs](src/typechecker.rs).


## Goals

The current roadmap of the program is:

- [x] Learn Chumsky
- [x] Use Chumsky to write the lexer.
- [x] Write AST and write a Typechecker for it.
- [x] Write parser.
- [ ] Write a ByteCode VM for the language.
- [ ] Finish parser (call expressions).
- [ ] Thoroughly test typechecker.
- [ ] Write REPL.

## How it works

This is a very simple typechecker with limited inference.

## Syntax

```rust
let x = "stringy";
let y = 1213;

funk collatz(n: int) -> int {
  if n == 1 {
    1
  } else if (n % 2) == 0{
    collatz(n / 2)
  } else {
    collatz(n * 3 + 1)
  }
}

print(collatz(7));
```
