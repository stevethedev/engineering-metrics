use std::time::Duration;

use deadpool_redis::Connection as RedisConnection;
use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};

use crate::Result;

/// A wrapper around a Redis connection.
pub struct Connection {
    connection: RedisConnection,
}

impl Connection {
    /// Create a new connection.
    ///
    /// # Arguments
    ///
    /// * `connection` - The Redis connection.
    pub(crate) fn new(connection: RedisConnection) -> Self {
        Self { connection }
    }

    /// Set a value in a Redis hashmap.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set a hash value under.
    /// * `field` - The hash field to set.
    /// * `value` - The value to set.
    ///
    /// # Errors
    ///
    /// Returns an error if the value could not be set.
    pub async fn hset<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        F: ToRedisArgs + Send + Sync + 'a,
        V: ToRedisArgs + Send + Sync + 'a,
    >(
        &mut self,
        key: K,
        field: F,
        value: V,
    ) -> Result<()> {
        self.connection.hset(key, field, value).await?;
        Ok(())
    }

    /// Set a value in the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set a value under.
    /// * `value` - The value to set.
    /// * `expiry` - The expiry time for the value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value could not be set.
    pub async fn set<'a, K: ToRedisArgs + Send + Sync + 'a, V: ToRedisArgs + Send + Sync + 'a>(
        &mut self,
        key: K,
        value: V,
        expiry: Option<&Duration>,
    ) -> Result<()> {
        let mut set = redis::cmd("SET");

        set.arg(&key).arg(&value);

        if let Some(expiry) = expiry {
            let seconds = expiry.as_secs();
            set.arg("EX").arg(seconds);
        }

        set.query_async(&mut self.connection).await?;

        Ok(())
    }

    /// Get a value from a Redis hashmap.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to get a hash value from.
    /// * `field` - The hash field to get.
    ///
    /// # Errors
    ///
    /// Returns an error if the value could not be retrieved.
    pub async fn hget<
        'a,
        K: ToRedisArgs + Send + Sync + 'a,
        F: ToRedisArgs + Send + Sync + 'a,
        RV: FromRedisValue,
    >(
        &mut self,
        key: K,
        field: F,
    ) -> Result<Option<RV>> {
        let value = self.connection.hget(key, field).await?;
        Ok(value)
    }

    /// Get a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to get a value from.
    ///
    /// # Errors
    ///
    /// Returns an error if the value could not be retrieved.
    pub async fn get<'a, K: ToRedisArgs + Send + Sync + 'a, RV: FromRedisValue>(
        &mut self,
        key: K,
    ) -> Result<RV> {
        let value = self.connection.get(key).await?;
        Ok(value)
    }

    /// Check if a key exists in the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check.
    ///
    /// # Errors
    ///
    /// Returns an error if the key could not be checked.
    pub async fn exists<'a, K: ToRedisArgs + Send + Sync + 'a>(&mut self, key: K) -> Result<bool> {
        let value = self.connection.exists(key).await?;
        Ok(value)
    }

    /// Delete a key from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to delete.
    ///
    /// # Errors
    ///
    /// Returns an error if the key could not be deleted.
    pub async fn delete<'a, K: ToRedisArgs + Send + Sync + 'a>(&mut self, key: K) -> Result<()> {
        self.connection.del(key).await?;
        Ok(())
    }

    /// Set an expiry on a key in the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set an expiry on.
    /// * `expiry` - The expiry time for the key.
    ///
    /// # Errors
    ///
    /// Returns an error if the expiry could not be set.
    pub async fn expire<'a, K: ToRedisArgs + Send + Sync + 'a>(
        &mut self,
        key: K,
        expiry: &Duration,
    ) -> Result<()> {
        let seconds = expiry.as_secs();
        redis::cmd("EXPIRE")
            .arg(&key)
            .arg(seconds)
            .query_async(&mut self.connection)
            .await?;

        Ok(())
    }
}
