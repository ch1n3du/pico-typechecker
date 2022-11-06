# pico-typechecker

This is a smol project I'm messing around with.
I've been thinking about how typecheckers work and what better way to learn about typecheckers than to write one.

Also I've been meaning to learn [Chumsky](https://github.com/zesterer/chumsky).

## Roadmap

The current roadmap of the program is:

- [x] Learn Chumsky
- [x] Use Chumsky to write the lexer.
- [x] Write AST and write  Typechecker for it.
- [x] Write basic parser.
- [ ] Finish parser (call expressions).
- [ ] Thoroughly test typechecker.
- [ ] Write tree-walking interpreter or ByteCode VM.
- [ ] Write REPL.

## How it works

This is a very simple typechecker with limited inference.

## Showcase

```rust
let x = "stringy";
let y = 1213;


funk collatz(n: int) -> int {
  print(n);

  if  n == 1 {
    1
  } else if (n % 2) == 0{
    collatz(n / 2)
  } else {
    collatz(n * 3 + 1)
  }
}
```

## Nov 2nd

Got basic parsing working
