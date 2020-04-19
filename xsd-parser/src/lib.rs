#![feature(in_band_lifetimes)]
#[cfg(test)]
extern crate log;
#[cfg(test)]
#[macro_use]
extern crate yaserde_derive;

pub mod abstract_code_model;
pub mod generator;
pub mod parser;
#[cfg(test)]
mod tests;
#[allow(dead_code)]
pub mod xml_to_xsd;
#[allow(dead_code)]
pub mod xsd_model;
pub mod xsd_to_abstract_code;
