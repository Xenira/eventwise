extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod repository;

#[proc_macro_derive(Aggregate, attributes(aggregate))]
pub fn derive_aggregate(input: TokenStream) -> TokenStream {
    derive_aggregate_internal(input.into()).into()
}

fn derive_aggregate_internal(input: TokenStream2) -> TokenStream2 {
    let input = parse_macro_input2!(input as syn::DeriveInput);

    repository::derive_aggregate(input).unwrap_or_else(|err| err.to_compile_error())
}

macro_rules! parse_macro_input2 {
    ($tokenstream:ident as $ty:ty) => {
        match syn::parse2::<$ty>($tokenstream) {
            Ok(data) => data,
            Err(err) => {
                return proc_macro2::TokenStream::from(err.to_compile_error());
            }
        }
    };
    ($tokenstream:ident) => {
        $crate::parse_macro_input!($tokenstream as _)
    };
}

pub(crate) use parse_macro_input2;

macro_rules! err {
    ($span:expr => $($msg:tt)*) => {
        ::syn::Error::new(::syn::spanned::Spanned::span(&$span), format!($($msg)*))
    };
    ($($msg:tt)*) => {
        ::syn::Error::new(::proc_macro2::Span::call_site(), format!($($msg)*))
    };
}

/// Bails out of a function with a syn error.
macro_rules! bail {
    ($span:expr => $($msg:tt)*) => {
        return Err($crate::err!($span => $($msg)*))
    };
    ($($msg:tt)*) => {
        return Err($crate::err!($($msg)*))
    };
}

macro_rules! ensure {
    ($cond:expr, $span:expr => $($msg:tt)*) => {
        if !$cond {
            $crate::bail!($span => $($msg)*);
        }
    };
    ($cond:expr, $($msg:tt)*) => {
        if !$cond {
            $crate::bail!($($msg)*);
        }
    };
}

pub(crate) use bail;
pub(crate) use ensure;
pub(crate) use err;

pub(crate) mod prelude {
    pub(crate) use crate::{bail, ensure, err};
    pub(crate) type Result<T> = std::result::Result<T, syn::Error>;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    // #[rustversion::attr(nightly, test)]
    #[test]
    #[allow(dead_code)]
    pub fn test_macrotest_expand() {
        macrotest::expand("tests/expand/*.rs");
    }

    #[test]
    fn test_expand() {
        for entry in glob::glob("tests/expand/*.rs").expect("Failed to read expand test glob") {
            let entry = entry.expect("Failed to read expand test file");
            runtime_expand_derive(&entry);
        }
    }

    fn runtime_expand_derive(path: &PathBuf) {
        let file = std::fs::File::open(path).expect("Failed to open expand test file");
        runtime_macros::emulate_derive_macro_expansion(
            file,
            &[("Aggregate", derive_aggregate_internal)],
        )
        .expect("Failed to expand derive macros in test file");
    }
}
