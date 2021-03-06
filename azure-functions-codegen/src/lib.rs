//! # Azure Functions for Rust
//!
//! This crate supports the code generation for the `azure-functions` crate.
#![feature(in_band_lifetimes)]
#![feature(proc_macro_diagnostic)]
#![feature(drain_filter)]
#![feature(try_from)]
#![recursion_limit = "128"]
#![deny(unused_extern_crates)]

#[macro_use]
extern crate lazy_static;
extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod export;
mod func;
mod util;

use proc_macro::TokenStream;

/// Implements the `export!` macro.
///
/// The `export!` macro is used to export a list of Azure Functions written
/// in Rust to the Azure Functions host.
///
/// This macro expects a comma-separated list of functions that have the
/// #[func] attribute applied.
///
/// # Examples
///
/// ```rust,ignore
/// pub fn main() {
///     azure_functions::worker_main(::std::env::args(), export!{
///         my_module::my_function
///     });
/// }
/// ```
#[proc_macro]
pub fn export(input: TokenStream) -> TokenStream {
    export::attr_impl(input)
}

/// Implements the `func` attribute.
///
/// This attribute is used to turn a Rust function into an Azure Function.
///
/// # Examples
///
/// ```rust,ignore
/// use azure_functions::func;
/// use azure_functions::bindings::HttpRequest;
///
/// #[func]
/// pub fn example(req: &HttpRequest) {
/// }
#[proc_macro_attribute]
pub fn func(args: TokenStream, input: TokenStream) -> TokenStream {
    func::attr_impl(args, input)
}
