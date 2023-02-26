use async_trait::async_trait;

/// The error type for the user repository.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The user repository is not available.")]
    NotAvailable,
    #[error("The user could not be created.")]
    CreateFailed,
    #[error("The user could not be deleted.")]
    DeleteFailed,
}

/// The result type for the user repository.
pub type Result<T> = std::result::Result<T, Error>;

/// The ID of a user.
pub type UserId = uuid::Uuid;

/// A single user-record.
pub struct User {
    /// The ID of the user.
    pub id: UserId,

    /// The username of the user.
    pub username: String,
}

/// The data to update a user.
pub struct UpdateUser {
    /// The new username of the user.
    pub username: Option<String>,
}

/// The data to create a user.
pub struct CreateUser<'a> {
    /// The username of the user.
    pub username: &'a str,

    /// The password of the user.
    pub password: &'a str,
}

/// The interface for the user repository.
#[async_trait]
pub trait Interface: Send + Sync {
    /// Checks for a user with the given username and password.
    ///
    /// # Parameters
    ///
    /// - `username`: The username to check.
    /// - `password`: The password to check.
    ///
    /// # Returns
    ///
    /// Returns a user if that user exists and the password is correct.
    ///
    /// # Errors
    ///
    /// Returns an error if the user repository is not available.
    async fn check_password(&self, username: &str, password: &str) -> Result<Option<User>>;

    /// Updates the password of a user.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user to update.
    /// - `password`: The new password of the user.
    ///
    /// # Errors
    ///
    /// Returns an error if the password could not be updated.
    async fn update_password(&self, id: UserId, password: &str) -> Result<()>;

    /// Creates a new user.
    ///
    /// # Parameters
    ///
    /// - `user`: The user to create.
    ///
    /// # Returns
    ///
    /// Returns the ID of the new user.
    ///
    /// # Errors
    ///
    /// Returns an error if the user could not be created.
    async fn create(&self, user: &CreateUser) -> Result<UserId>;

    /// Retrieves a user by ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user to retrieve.
    ///
    /// # Returns
    ///
    /// Returns the user if it exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the user repository is not available.
    async fn get(&self, id: UserId) -> Result<Option<User>>;

    /// Retrieves a user by username.
    ///
    /// # Parameters
    ///
    /// - `username`: The username of the user to retrieve.
    ///
    /// # Returns
    ///
    /// Returns the user if it exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the user repository is not available.
    async fn get_by_username(&self, username: &str) -> Result<Option<User>>;

    /// Deletes a user by ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user to delete.
    ///
    /// # Errors
    ///
    /// Returns an error if the user could not be deleted.
    async fn delete(&self, id: UserId) -> Result<()>;

    /// Updates a user.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the user to update.
    /// - `user`: The user to update.
    ///
    /// # Errors
    ///
    /// Returns an error if the user could not be updated.
    async fn update(&self, id: UserId, user: &UpdateUser) -> Result<()>;
}
