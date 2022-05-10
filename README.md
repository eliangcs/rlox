# Yet Another Lox Impelementation in Rust

## Building

```sh
# Build with debug features and run in REPL mode
cargo run --features debug

# Build with debug features and run a file
cargo run --features debug -- test.lox
```

## Debugging

```console
$ cat debug.txt
b expression
b binary
b grouping
b number
b unary
b parse_precedence

$ cat test.lox
(-1 + 2) * 3 - -4

$ rust-lldb -S debug ./target/debug/rlox test.lox
```
