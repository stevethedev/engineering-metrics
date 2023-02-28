use async_trait::async_trait;

use lib_crypto::{hash_password, verify_password};
use lib_database::{
    Connection, UserCredentialsController, UserCredentialsFilter, UserCredentialsWrite,
};

use crate::{
    user_repo::{CreateUser, UpdateUser},
    User, UserId,
};

pub struct Repo {
    controller: UserCredentialsController,
}

impl Repo {
    pub fn new(connection: Connection) -> Self {
        let controller = UserCredentialsController::new(connection);
        Self { controller }
    }
}

#[async_trait]
impl super::Interface for Repo {
    async fn check_password(
        &self,
        username: &str,
        password: &str,
    ) -> crate::user_repo::Result<Option<User>> {
        let user = self
            .controller
            .read(UserCredentialsFilter::default().username(username.to_string()))
            .await
            .map_err(|_| super::Error::NotAvailable)?;

        let password_hash = user
            .as_ref()
            .map(|u| u.password_hash.to_string())
            .unwrap_or_default();

        let is_valid = verify_password(password.as_bytes(), &password_hash).unwrap_or(false);

        if is_valid {
            Ok(user.map(|u| User {
                id: u.id,
                username: u.username,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_password(&self, id: UserId, password: &str) -> crate::user_repo::Result<()> {
        let write = UserCredentialsWrite::default().password_hash(password.to_string());
        self.controller
            .update(id, write)
            .await
            .map_err(|_| super::Error::NotAvailable)?;
        Ok(())
    }

    async fn create(&self, user: &CreateUser) -> crate::user_repo::Result<UserId> {
        let password_hash = hash_password(user.password.as_bytes()).map_err(|e| {
            log::error!("Failed to hash password: {}", e);
            super::Error::NotAvailable
        })?;
        let write = UserCredentialsWrite::default()
            .username(user.username.to_string())
            .password_hash(password_hash)
            .enabled(true);
        let id = self
            .controller
            .create(write)
            .await
            .map_err(|_| super::Error::NotAvailable)?;
        Ok(id)
    }

    async fn get(&self, id: UserId) -> crate::user_repo::Result<Option<User>> {
        let filter = UserCredentialsFilter::default().id(id);

        let user = self
            .controller
            .read(filter)
            .await
            .map_err(|_| super::Error::NotAvailable)?;

        Ok(user.map(|u| User {
            id: u.id,
            username: u.username,
        }))
    }

    async fn get_by_username(&self, username: &str) -> crate::user_repo::Result<Option<User>> {
        let filter = UserCredentialsFilter::default().username(username.to_string());

        let user = self
            .controller
            .read(filter)
            .await
            .map_err(|_| super::Error::NotAvailable)?;

        Ok(user.map(|u| User {
            id: u.id,
            username: u.username,
        }))
    }

    async fn delete(&self, id: UserId) -> crate::user_repo::Result<()> {
        self.controller
            .delete(id)
            .await
            .map_err(|_| super::Error::NotAvailable)?;
        Ok(())
    }

    async fn update(&self, id: UserId, user: &UpdateUser) -> crate::user_repo::Result<()> {
        let write = UserCredentialsWrite::default();

        let write = if let Some(username) = &user.username {
            write.username(username.to_string())
        } else {
            write
        };

        self.controller
            .update(id, write)
            .await
            .map_err(|_| super::Error::NotAvailable)?;
        Ok(())
    }
}
