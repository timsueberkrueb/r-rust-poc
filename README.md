# Proof of Concept: Calling Rust from R

**NOTE**: this is just a proof of concept, not a usable crate collection! You have been warned.

## Usage

`r-bindgen` provides a procedural attribute macro generating a suitable FFI function for R-Rust-interop:

```rust
#[r_bindgen]
pub fn rust_sort(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
}
```

In R, we just need to load the generated shared library (use `crate-type = ["cdylib"]` in your `Cargo.toml`) and `.Call` the foreign function:

```R
dyn.load("../target/debug/libexample.so")
v <- sample(1:64, 64)
.Call("rust_sort", v)
```

Awesome, isn't it?

See the `example` folder for a working Cargo project using `r-bindgen`.

## Crate Overview

* [`example`](example/): Example crate
* [`r_bindgen`](r-bindgen/): Procedural macro for generating suitable FFI functions
* [`r_internals`](r-internals/): Generated Rust bindings for `Rinternals.h`, implementation detail, should not be used directly
* [`r_interop`](r-interop/): Conversion between R and Rust types

## License

This project is licensed under either of the following licenses, at your option:

* [Apache License Version 2.0](LICENSE-APACHE)
* [MIT License](LICENSE-MIT)
