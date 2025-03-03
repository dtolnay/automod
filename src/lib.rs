//! [![github]](https://github.com/dtolnay/automod)&ensp;[![crates-io]](https://crates.io/crates/automod)&ensp;[![docs-rs]](https://docs.rs/automod)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
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
//! The macro takes an optional visibility to apply on the generated modules:
//! `automod::dir!(pub "path/to/directory")`.
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

#![doc(html_root_url = "https://docs.rs/automod/1.0.15")]
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
    let mut module_name = name.replace('-', "_");
    if module_name.starts_with(|ch: char| ch.is_ascii_digit()) {
        module_name.insert(0, '_');
    }

    let path = Option::into_iter(if name == module_name {
        None
    } else {
        Some(format!("{}.rs", name))
    });

    let ident = Ident::new(&module_name, Span::call_site());

    quote! {
        #(#[path = #path])*
        #vis mod #ident;
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
