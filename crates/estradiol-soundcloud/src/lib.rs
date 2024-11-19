#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![forbid(unsafe_code)]

mod client;
pub use client::Client;
mod error;
pub use error::Error;
pub mod endpoints;
pub mod models;
