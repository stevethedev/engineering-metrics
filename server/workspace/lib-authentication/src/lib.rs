#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

mod controllers;
mod error;
mod provider;
mod token;
mod token_repo;
mod user_repo;

pub use controllers::login::Credentials;
pub use error::{Error, Result};
pub use provider::{Interface as ProviderInterface, Provider};
pub use token::Token;
pub use token_repo::{
    Error as TokenRepoError, Interface as TokenRepoInterface, Memory as MemoryTokenRepo,
    Result as TokenRepoResult, TokenRepo,
};
pub use user_repo::{
    Error as UserRepoError, Interface as UserRepoInterface, Memory as MemoryUserRepo,
    Repo as UserRepo, Result as UserRepoResult, User, UserId,
};
