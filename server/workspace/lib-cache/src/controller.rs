use deadpool_redis::{Config, Runtime};

use crate::connection::Connection;
use crate::Result;

/// A controller for a Redis cache.
#[derive(Clone)]
pub struct Controller {
    pool: deadpool_redis::Pool,
}

impl std::fmt::Debug for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Controller").finish()
    }
}

impl Controller {
    /// Create a new controller.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to connect to.
    ///
    /// # Errors
    ///
    /// Returns an error if the controller could not be created.
    pub fn open(url: &str) -> Result<Self> {
        let cfg = Config::from_url(url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        Ok(Self { pool })
    }

    /// Get a connection to the cache.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection could not be established.
    pub async fn connect(&self) -> Result<Connection> {
        let client = self.pool.get().await?;
        Ok(Connection::new(client))
    }
}
