Testing compile time regression on Rust 1.57.0
https://github.com/rust-lang/rust/issues/91598


How to observe:

Compile with 1.56.1:
```
$ rustup install 1.56.1
$ cargo clean ; time cargo +1.56.1 build
```

Compile with 1.57.0:
```
$ rustup install 1.57.0
$ cargo clean ; time cargo +1.57.0 build
```

on a AMD Ryzen 7 1700, approx times should be:
- 1.56.1: ~59s
- 1.57.0: ~1'50s

Adding more nesting in the router (see comment in `main.rs`) should cause compile time to diverge and have the 1.57.0 version increase more rapidly than the 1.56.1
