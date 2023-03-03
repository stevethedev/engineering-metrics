#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

pub use self::{
    connection::{Connection, LevelFilter, Options},
    controllers::user_credentials::{
        Controller as UserCredentialsController, Filter as UserCredentialsFilter,
        Write as UserCredentialsWrite,
    },
    entities::user_credentials::Model as UserCredentials,
    error::{Error, Result},
};

mod connection;
mod controllers;
mod entities;
mod error;
