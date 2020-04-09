#[cfg(test)]
#[macro_use]
extern crate log;
#[cfg(test)]
#[macro_use]
extern crate yaserde_derive;

pub mod generator;
pub mod parser;
#[allow(dead_code)]
pub mod xsd_model;
#[cfg(test)]
mod tests;
