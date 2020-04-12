#[cfg(test)]
extern crate log;
#[cfg(test)]
#[macro_use]
extern crate yaserde_derive;

pub mod generator;
pub mod parser;
#[cfg(test)]
mod tests;
#[allow(dead_code)]
pub mod xml_to_xsd;
#[allow(dead_code)]
pub mod xsd_model;
