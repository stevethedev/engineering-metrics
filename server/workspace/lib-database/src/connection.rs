use std::{borrow::Cow, time::Duration};

pub use log::LevelFilter;
use sea_orm::ConnectOptions;

use lib_database_migration::{Migrator, MigratorTrait};

use crate::{Error, Result};

/// A database connection controller.
#[derive(Clone)]
pub struct Connection {
    connection: sea_orm::DatabaseConnection,
}

impl Connection {
    /// Open a connection to the database.
    ///
    /// # Arguments
    ///
    /// * `options` - The database connection options.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection could not be established.
    pub async fn connect(options: Options) -> Result<Self> {
        let connection = sea_orm::Database::connect(options.options)
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;
        Ok(Self { connection })
    }

    /// Run any pending database migrations.
    ///
    /// # Errors
    ///
    /// Returns an error if the migrations could not be applied.
    pub async fn migrate(&self) -> Result<()> {
        Migrator::up(&self.connection, None)
            .await
            .map_err(|e| Error::Migration(e.to_string()))?;
        Ok(())
    }

    /// Close the connection controller.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection could not be closed.
    pub async fn disconnect(self) -> Result<()> {
        self.connection
            .close()
            .await
            .map_err(|e| Error::Connection(e.to_string()))?;
        Ok(())
    }
}

/// A database connection configuration object.
pub struct Options {
    options: ConnectOptions,
}

impl Options {
    /// Create a new configuration options struct for a database by passing in a URI string.
    pub fn new(url: String) -> Self {
        Self {
            options: ConnectOptions::new(url),
        }
    }

    /// Set the maximum number of connections in the pool.
    pub fn with_max_connections(&mut self, value: u32) -> &mut Self {
        self.options.max_connections(value);
        self
    }

    /// Set the minimum number of connections in the pool.
    pub fn with_min_connections(&mut self, value: u32) -> &mut Self {
        self.options.min_connections(value);
        self
    }

    /// Set the timeout duration when acquiring a connection.
    pub fn with_connect_timeout(&mut self, value: Duration) -> &mut Self {
        self.options.connect_timeout(value);
        self
    }

    /// Set the idle duration before closing a connection.
    pub fn with_idle_timeout(&mut self, value: Duration) -> &mut Self {
        self.options.idle_timeout(value);
        self
    }

    /// Set the maximum amount of time to spend waiting for acquiring a connection.
    pub fn with_acquire_timeout(&mut self, value: Duration) -> &mut Self {
        self.options.acquire_timeout(value);
        self
    }

    /// Set the maximum lifetime of individual connections.
    pub fn with_max_lifetime(&mut self, lifetime: Duration) -> &mut Self {
        self.options.max_lifetime(lifetime);
        self
    }

    /// Set whether statement-logging is enabled (default: true).
    pub fn with_sql_logging(&mut self, value: bool) -> &mut Self {
        self.options.sqlx_logging(value);
        self
    }

    /// Set the log-level for SQL statement logging (default: INFO).
    pub fn with_sql_logging_level(&mut self, level: LevelFilter) -> &mut Self {
        self.options.sqlx_logging_level(level);
        self
    }

    /// Set the SQLCipher key.
    pub fn with_sqlcipher_key<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<Cow<'static, str>>,
    {
        self.options.sqlcipher_key(value);
        self
    }

    /// Set the PostgreSQL schema search path.
    pub fn with_schema_search_path(&mut self, schema_search_path: String) -> &mut Self {
        self.options.set_schema_search_path(schema_search_path);
        self
    }
}
