#[macro_use]
extern crate serde_derive;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate hyper_native_tls;



#[cfg(test)]
mod test;


pub mod lang;
pub mod client;
pub mod structure;
