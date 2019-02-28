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
        Ok(names) => {
            names.into_iter().map(mod_item).collect()
        }
        Err(err) => {
            syn::Error::new(Span::call_site(), err).to_compile_error()
        }
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
        if file_name == "mod.rs" {
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

    names.sort();
    Ok(names)
}
