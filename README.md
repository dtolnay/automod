Automod
=======

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/automod-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/automod)
[<img alt="crates.io" src="https://img.shields.io/crates/v/automod.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/automod)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-automod-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/automod)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/dtolnay/automod/CI/master?style=for-the-badge" height="20">](https://github.com/dtolnay/automod/actions?query=branch%3Amaster)

Pull in every source file in a directory as a module.

```toml
[dependencies]
automod = "1.0"
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
Automod solves this by adding *tests/regression.rs* containing:

```rust
mod regression {
    automod::dir!("tests/regression");
}
```

The macro invocation expands to:

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
