# pico-typechecker

This is a smol project I'm messing around with.
I've been thinking about how typecheckers work and what better way to learn about typecheckers than to write one.

Also I've been meaning to learn [Chumsky](https://github.com/zesterer/chumsky).

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

## Showcase

```rust
let x = "stringy";
let y = 1213;


funk collatz(n: int) -> int {
  print(n);

  if n == 1 {
    1
  } else if (n % 2) == 0{
    collatz(n / 2)
  } else {
    collatz(n * 3 + 1)
  }
}
```

```python
import os
from os import path
def file_structure(c:/chinedu/documents/music):

# If it's a file return the string
```

## Nov 2nd

Got basic parsing working

## Update \[21/11/22\]

This project has kind of grown in scope, the current plan is to finish the ByteCode VM to allow faster iteration.
Also I'm getting a better grip on how type-inference and might do more on that in future.
