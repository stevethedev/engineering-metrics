use std::env::VarError;

/// Environment variable interface.
pub trait EnvironmentVariable<T> {
    /// The name of the environment variable.
    const NAME: &'static str;

    /// The default value of the environment variable.
    fn default() -> T;

    /// Get the raw value of the environment variable.
    fn get_raw() -> Result<String, VarError> {
        const VAR_PREFIX: Option<&str> = option_env!("VAR_PREFIX");
        let prefix = VAR_PREFIX.unwrap_or("APP_");
        std::env::var(format!("{prefix}{}", Self::NAME))
    }

    /// Get the value of the environment variable.
    fn get() -> T;
}

/// The size of the authentication token.
pub struct AuthTokenSize;
impl EnvironmentVariable<usize> for AuthTokenSize {
    const NAME: &'static str = "AUTH_TOKEN_SIZE";

    fn default() -> usize {
        32
    }

    fn get() -> usize {
        match Self::get_raw() {
            Ok(value) => value.parse().ok().unwrap_or_else(Self::default),
            Err(_) => Self::default(),
        }
    }
}

/// The time to live of the authentication token.
pub struct AuthTokenTtl;
impl EnvironmentVariable<u64> for AuthTokenTtl {
    const NAME: &'static str = "AUTH_TOKEN_TTL";

    fn default() -> u64 {
        // 1 hour
        3600
    }

    fn get() -> u64 {
        match Self::get_raw() {
            Ok(value) => value.parse().ok().unwrap_or_else(Self::default),
            Err(_) => Self::default(),
        }
    }
}

/// The connection string to use to connect to the database.
pub struct DbConnectionString;
impl EnvironmentVariable<String> for DbConnectionString {
    const NAME: &'static str = "DB_CONNECTION_STRING";

    fn default() -> String {
        "sqlite://database.db".to_string()
    }

    fn get() -> String {
        match Self::get_raw() {
            Ok(value) => value,
            Err(_) => Self::default(),
        }
    }
}

/// The file path to the encryption key. This can hold any value of any length.
/// If the file does not exist, it will be created. If the file is empty, a new
/// key will be generated.
pub struct EncryptionKeyPath;

impl EnvironmentVariable<String> for EncryptionKeyPath {
    const NAME: &'static str = "ENCRYPTION_KEY_PATH";

    fn default() -> String {
        "encryption.key".to_string()
    }

    fn get() -> String {
        match Self::get_raw() {
            Ok(value) => value,
            Err(_) => Self::default(),
        }
    }
}

/// The size of the refresh token.
pub struct RefreshTokenSize;
impl EnvironmentVariable<usize> for RefreshTokenSize {
    const NAME: &'static str = "REFRESH_TOKEN_SIZE";

    fn default() -> usize {
        32
    }

    fn get() -> usize {
        match Self::get_raw() {
            Ok(value) => value.parse().ok().unwrap_or_else(Self::default),
            Err(_) => Self::default(),
        }
    }
}

/// The time to live of the refresh token.
pub struct RefreshTokenTtl;
impl EnvironmentVariable<u64> for RefreshTokenTtl {
    const NAME: &'static str = "REFRESH_TOKEN_TTL";

    fn default() -> u64 {
        // 7 days
        604800
    }

    fn get() -> u64 {
        match Self::get_raw() {
            Ok(value) => value.parse().ok().unwrap_or_else(Self::default),
            Err(_) => Self::default(),
        }
    }
}
