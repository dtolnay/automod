//! [![github]](https://github.com/dtolnay/automod)&ensp;[![crates-io]](https://crates.io/crates/automod)&ensp;[![docs-rs]](https://docs.rs/automod)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! **Pull in every source file in a directory as a module.**
//!
//! # Syntax
//!
//! ```
//! # const IGNORE: &str = stringify! {
//! automod::dir!("path/to/directory");
//! # };
//! ```
//!
//! This macro expands to one or more `mod` items, one for each source file in
//! the specified directory.
//!
//! The path is given relative to the directory containing Cargo.toml.
//!
//! It is an error if the given directory contains no source files.
//!
//! # Example
//!
//! Suppose that we would like to keep a directory of regression tests for
//! individual numbered issues:
//!
//! - tests/
//!   - regression/
//!     - issue1.rs
//!     - issue2.rs
//!     - ...
//!     - issue128.rs
//!
//! We would like to be able to toss files in this directory and have them
//! automatically tested, without listing them in some explicit list of modules.
//! Automod solves this by adding *tests/regression.rs* containing:
//!
//! ```
//! # const IGNORE: &str = stringify! {
//! mod regression {
//!     automod::dir!("tests/regression");
//! }
//! # };
//! ```
//!
//! The macro invocation expands to:
//!
//! ```
//! # const IGNORE: &str = stringify! {
//! mod issue1;
//! mod issue2;
//! /* ... */
//! mod issue128;
//! # };
//! ```

#![allow(clippy::enum_glob_use, clippy::needless_pass_by_value)]

extern crate proc_macro;

mod error;

use crate::error::{Error, Result};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, LitStr, Visibility};

struct Arg {
    vis: Visibility,
    path: LitStr,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Arg {
            vis: input.parse()?,
            path: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn dir(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Arg);
    let vis = &input.vis;
    let rel_path = input.path.value();

    let dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(manifest_dir) => PathBuf::from(manifest_dir).join(rel_path),
        None => PathBuf::from(rel_path),
    };

    let expanded = match source_file_names(dir) {
        Ok(names) => names.into_iter().map(|name| mod_item(vis, name)).collect(),
        Err(err) => syn::Error::new(Span::call_site(), err).to_compile_error(),
    };

    TokenStream::from(expanded)
}

fn mod_item(vis: &Visibility, name: String) -> TokenStream2 {
    if name.contains('-') {
        let path = format!("{}.rs", name);
        let ident = Ident::new(&name.replace('-', "_"), Span::call_site());
        quote! {
            #[path = #path]
            #vis mod #ident;
        }
    } else {
        let ident = Ident::new(&name, Span::call_site());
        quote! {
            #vis mod #ident;
        }
    }
}

fn source_file_names<P: AsRef<Path>>(dir: P) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let mut failures = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let file_name = entry.file_name();
        if file_name == "mod.rs" || file_name == "lib.rs" || file_name == "main.rs" {
            continue;
        }

        let path = Path::new(&file_name);
        if path.extension() == Some(OsStr::new("rs")) {
            match file_name.into_string() {
                Ok(mut utf8) => {
                    utf8.truncate(utf8.len() - ".rs".len());
                    names.push(utf8);
                }
                Err(non_utf8) => {
                    failures.push(non_utf8);
                }
            }
        }
    }

    failures.sort();
    if let Some(failure) = failures.into_iter().next() {
        return Err(Error::Utf8(failure));
    }

    if names.is_empty() {
        return Err(Error::Empty);
    }

    names.sort();
    Ok(names)
}
