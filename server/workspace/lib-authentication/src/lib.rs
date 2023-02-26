#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

pub use self::{
    controllers::{
        login::{Credentials as LoginCredentials, TokenPair},
        register::Credentials as RegisterCredentials,
    },
    data::{AuthToken, Interface as TokenInterface, RefreshToken},
    error::{Error, Result},
    provider::{Interface as ProviderInterface, Provider},
    token_repo::{
        Error as TokenRepoError, Interface as TokenRepoInterface, Memory as MemoryTokenRepo,
        Result as TokenRepoResult, TokenRepo,
    },
    user_repo::{
        Error as UserRepoError, Interface as UserRepoInterface, Memory as MemoryUserRepo,
        Repo as UserRepo, Result as UserRepoResult, User, UserId,
    },
};

mod controllers;
mod data;
mod error;
mod provider;
mod token_repo;
mod user_repo;
