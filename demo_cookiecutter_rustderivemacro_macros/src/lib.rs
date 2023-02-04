extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{parse_macro_input, DeriveInput};

mod func;

/// A simple derive macro to demonstrate the output of cookiecutter-rustderivemacro.
///
/// Example
/// -------
///
/// ```ignore
/// use demo_cookiecutter_rustderivemacro::*;
///
/// #[derive(DemoDerive)]
/// pub struct MyStruct {
///     _populate_me: i32,
/// }
/// ```
///
/// will Expand into:
///
/// ```ignore
/// use demo_cookiecutter_rustderivemacro::DemoDerive;
///
/// // POPULATE ME
/// ```
#[proc_macro_derive(DemoDerive, attributes(__change_me__))]
pub fn demo_cookiecutter_rustderivemacro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // =================================================================================
    // Build the output
    let expanded = quote! {
        use demo_cookiecutter_rustderivemacro_types::DemoDerive;
        impl demo_cookiecutter_rustderivemacro_types::DemoDerive for #name {

            /// Sample function that always return a fixed value.
            fn test_demo_cookiecutter_rustderivemacro(&self) -> i32 {
                42069
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
