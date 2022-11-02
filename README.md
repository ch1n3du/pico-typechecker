# pico-typechecker

This is a smol project I'm messing around with.
I've been thinking about how typecheckers work and what better way to learn about typecheckers than to write one.

Also I've been meaning to learn [Chumsky](https://github.com/zesterer/chumsky).

## How it works

This is a very simple typechecker with limited inference.

## Showcase

```rust
// Type inference for literals
let x = "stringy";
let y = 1213;

if x == y {
    print("Hello")
} else {
    printy("q23")
}

funk fib(n: number) {
    if n < 2 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}
```
