# Proof of Concept: Calling Rust from R

**NOTE**: this is just a proof of concept, not a usable crate collection! You have been warned.

## Requirements

This project has been tested in the following environment:

* Ubuntu 18.04
* R = 3.6.1
* Rust 1.39.0 (cargo 1.39.0)

Contributions welcome for making this work on other platforms (e.g. Windows).

## Usage

`r-bindgen` provides a procedural attribute macro generating a suitable FFI function for R-Rust-interop:

```rust
#[r_bindgen]
pub fn rust_sort(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
}
```

Awesome, isn't it?

## Playground

Playground contains a Rust crate that produces a shared library.

In R, we just need to load the generated shared library (use `crate-type = ["cdylib"]` in your `Cargo.toml`) and `.Call` the foreign function:

```R
dyn.load("../target/debug/libplayground.so")
v <- sample(1:64, 64)
.Call("rust_sort", v)
```

You can run the demo from the `playground` folder using:

```bash
cargo build
R -f R/hello.R
```

## Example package

The `example` folder demonstrates how a Rust crate can be turned into an R package.

If you want to include documentation, run the R prompt from the `example` folder:

```R
roxygen2::roxygenise()
```

From the root of the repository, run:

```bash
R CMD build example
```

Then install the package, e.g. using the `R` REPL:

```R
install.packages("rustexample_0.1.0.tar.gz")
# You may wish to specify the install location using the lib.loc parameter:
# install.packages("rustexample_0.1.0.tar.gz", lib.loc="path/to/R/libraries")
```

To try the package, run using the `R` package:

```R
> library("rustexample")
> rust.sort(c(5, 3, 1, 4, 2))
[1] 1 2 3 4 5
> rust.mean(rnorm(64, mean=5, sd=1))
[1] 5.045658
```

## Crate Overview

* [`example`](example/): Example crate/R package which can be installed
* [`playground`](playground/): Example loading a shared library directly
* [`r_bindgen`](r-bindgen/): Procedural macro for generating suitable FFI functions
* [`r_internals`](r-internals/): Generated Rust bindings for `Rinternals.h`, implementation detail, should not be used directly
* [`r_interop`](r-interop/): Conversion between R and Rust types

## License

This project is licensed under either of the following licenses, at your option:

* [Apache License Version 2.0](LICENSE-APACHE)
* [MIT License](LICENSE-MIT)
