use super::{CreateUser, Error, Interface, Result, UpdateUser, User, UserId};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A user repository that stores all users in memory.
struct Record {
    username: String,
    password: String,
}

/// A user repository that stores all users in memory.
pub struct Repo {
    users: Arc<RwLock<HashMap<uuid::Uuid, Record>>>,
}

impl Default for Repo {
    /// Creates a new in-memory user repository.
    fn default() -> Self {
        let mut users = HashMap::new();
        users.insert(
            uuid::Uuid::new_v4(),
            Record {
                username: "admin".to_string(),
                password: "admin".to_string(),
            },
        );
        Self {
            users: Arc::new(RwLock::new(users)),
        }
    }
}

#[async_trait]
impl Interface for Repo {
    async fn check_password(&self, username: &str, password: &str) -> Result<Option<User>> {
        let users = self.users.read().map_err(|_| Error::NotAvailable)?;
        for (id, user) in users.iter() {
            if user.username == username && user.password == password {
                return Ok(Some(User {
                    id: *id,
                    username: user.username.clone(),
                }));
            }
        }
        Ok(None)
    }

    async fn update_password(&self, id: UserId, password: &str) -> Result<()> {
        let mut users = self.users.write().map_err(|_| Error::NotAvailable)?;
        if let Some(user) = users.get_mut(&id) {
            user.password = password.to_string();
        }
        Ok(())
    }

    async fn create(&self, user: &CreateUser) -> Result<UserId> {
        let mut users = self.users.write().map_err(|_| Error::NotAvailable)?;
        let id = uuid::Uuid::new_v4();
        users.insert(
            id,
            Record {
                username: user.username.to_string(),
                password: user.password.to_string(),
            },
        );
        Ok(id)
    }

    async fn get(&self, id: UserId) -> Result<Option<User>> {
        let users = self.users.read().map_err(|_| Error::NotAvailable)?;
        if let Some(user) = users.get(&id) {
            Ok(Some(User {
                id,
                username: user.username.clone(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>> {
        let users = self.users.read().map_err(|_| Error::NotAvailable)?;
        for (id, user) in users.iter() {
            if user.username == username {
                return Ok(Some(User {
                    id: *id,
                    username: user.username.clone(),
                }));
            }
        }
        Ok(None)
    }

    async fn delete(&self, id: UserId) -> Result<()> {
        let mut users = self.users.write().map_err(|_| Error::NotAvailable)?;
        users.remove(&id);
        Ok(())
    }

    async fn update(&self, id: UserId, updates: &UpdateUser) -> Result<()> {
        let mut users = self.users.write().map_err(|_| Error::NotAvailable)?;
        if let Some(user) = users.get_mut(&id) {
            if let Some(ref username) = updates.username {
                user.username = username.to_string();
            }
        }
        Ok(())
    }
}
