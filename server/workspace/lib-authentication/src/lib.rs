#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

mod error;
mod login;
mod token;

pub use error::{Error, Result};
pub use login::{login, Login};
pub use token::Token;
