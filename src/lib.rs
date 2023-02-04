//! This library provides a simple derive macro to map Enum variants to demonstrate the output of cookiecutter-rustderivemacro.
//!
//! Example
//! -------
//!
//! ```rust
//! use demo_cookiecutter_rustderivemacro::*;
//!
//!
//! #[derive(DemoDerive)]
//! struct MyStruct {
//!     _populate_me: i32,
//! }
//!
//! assert!(
//!     // Write a test here
//!     true
//! );
//!
//! ```
//!
pub mod prelude;

pub use demo_cookiecutter_rustderivemacro_macros::*;
pub use demo_cookiecutter_rustderivemacro_types::*;

mod tests;
