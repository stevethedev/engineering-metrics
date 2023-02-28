use async_trait::async_trait;

pub use interface::{CreateUser, Error, Interface, Result, UpdateUser, User, UserId};
use lib_database::Connection;
pub use memory::Repo as Memory;

mod database;
mod interface;
mod memory;

/// The master user repository.
#[derive(Clone)]
pub struct Repo {
    repo: std::sync::Arc<Box<dyn Interface>>,
}

impl Repo {
    /// Creates a new in-memory user repository.
    #[must_use]
    pub fn memory() -> Self {
        let repo = memory::Repo::default();
        Self {
            repo: std::sync::Arc::new(Box::new(repo)),
        }
    }

    /// Creates a new database user repository.
    #[must_use]
    pub fn database(connection: Connection) -> Self {
        let repo = database::Repo::new(connection);
        Self {
            repo: std::sync::Arc::new(Box::new(repo)),
        }
    }
}

#[async_trait]
impl Interface for Repo {
    async fn check_password(&self, username: &str, password: &str) -> Result<Option<User>> {
        self.repo.check_password(username, password).await
    }

    async fn update_password(&self, id: UserId, password: &str) -> Result<()> {
        self.repo.update_password(id, password).await
    }

    async fn create(&self, user: &CreateUser) -> Result<UserId> {
        self.repo.create(user).await
    }

    async fn get(&self, id: UserId) -> Result<Option<User>> {
        self.repo.get(id).await
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        self.repo.get_by_username(username).await
    }

    async fn delete(&self, id: UserId) -> Result<()> {
        self.repo.delete(id).await
    }

    async fn update(&self, id: UserId, user: &UpdateUser) -> Result<()> {
        self.repo.update(id, user).await
    }
}
