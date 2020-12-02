#[cfg_attr(test, macro_use)]
extern crate yaserde_derive;

pub mod types;
pub mod utils;

// Required for macro-generated code to find this crate under its name.
extern crate self as xsd_types;
