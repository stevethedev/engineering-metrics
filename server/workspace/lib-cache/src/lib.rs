#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

pub use connection::Connection;
pub use controller::Controller;
pub use error::{Error, Result};

mod connection;
mod controller;
mod error;
