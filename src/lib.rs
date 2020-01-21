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
//! Automod solves this by adding *tests/regression/mod.rs* containing:
//!
//! ```
//! # const IGNORE: &str = stringify! {
//! automod::dir!("tests/regression");
//! # };
//! ```
//!
//! The invocation expands to:
//!
//! ```
//! # const IGNORE: &str = stringify! {
//! mod issue1;
//! mod issue2;
//! /* ... */
//! mod issue128;
//! # };
//! ```

extern crate proc_macro;

mod error;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, LitStr};

use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::error::{Error, Result};

#[proc_macro]
pub fn dir(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let path = input.value();

    let expanded = match source_file_names(path) {
        Ok(names) => names.into_iter().map(mod_item).collect(),
        Err(err) => syn::Error::new(Span::call_site(), err).to_compile_error(),
    };

    TokenStream::from(expanded)
}

fn mod_item(name: String) -> TokenStream2 {
    let ident = Ident::new(&name, Span::call_site());
    quote! {
        mod #ident;
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
