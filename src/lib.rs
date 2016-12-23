#![feature(proc_macro)]
#![feature(custom_attribute)]

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod serialization_tests;

extern crate serde;
pub mod apiai;
