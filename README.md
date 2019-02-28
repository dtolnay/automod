Automod
=======

[![Build Status](https://api.travis-ci.com/dtolnay/automod.svg?branch=master)](https://travis-ci.org/dtolnay/automod)
[![Latest Version](https://img.shields.io/crates/v/automod.svg)](https://crates.io/crates/automod)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/automod)

Pull in every source file in a directory as a module.

```toml
[dependencies]
automod = "0.1"
```

## Syntax

```rust
automod::dir!("path/to/directory");
```

This macro expands to one or more `mod` items, one for each source file in the
specified directory.

The path is given relative to the directory containing Cargo.toml.

It is an error if the given directory contains no source files.

## Example

Suppose that we would like to keep a directory of regression tests for
individual numbered issues:

- tests/
  - regression/
    - issue1.rs
    - issue2.rs
    - ...
    - issue128.rs

We would like to be able to toss files in this directory and have them
automatically tested, without listing them in some explicit list of modules.
Automod solves this by adding *tests/regression/mod.rs* containing:

```rust
automod::dir!("tests/regression");
```

The invocation expands to:

```rust
mod issue1;
mod issue2;
/* ... */
mod issue128;
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
