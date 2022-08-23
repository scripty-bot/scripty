#![feature(string_remove_matches)]

mod auth;
mod endpoints;
mod entrypoint;
mod errors;

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate serde;

pub use entrypoint::entrypoint;
