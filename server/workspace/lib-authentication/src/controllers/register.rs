use crate::user_repo::CreateUser;
use crate::{Result, UserId, UserRepoInterface};

/// Registration credentials.
pub struct Credentials<'a> {
    /// The username.
    pub username: &'a str,

    /// The password.
    pub password: &'a str,
}

pub async fn register<'a>(
    user_repo: &impl UserRepoInterface,
    credentials: &Credentials<'a>,
) -> Result<UserId> {
    let user = CreateUser {
        username: credentials.username,
        password: credentials.password,
    };

    Ok(user_repo.create(&user).await?)
}
