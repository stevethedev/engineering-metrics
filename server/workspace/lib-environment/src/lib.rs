use std::env::VarError;

/// Environment variable interface.
pub trait EnvironmentVariable<T> {
    /// The name of the environment variable.
    const NAME: &'static str;

    /// The default value of the environment variable.
    const DEFAULT: T;

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
    const DEFAULT: usize = 32;

    fn get() -> usize {
        match Self::get_raw() {
            Ok(value) => value.parse().unwrap_or(Self::DEFAULT),
            Err(_) => Self::DEFAULT,
        }
    }
}

/// The time to live of the authentication token.
pub struct AuthTokenTtl;
impl EnvironmentVariable<u64> for AuthTokenTtl {
    const NAME: &'static str = "AUTH_TOKEN_TTL";
    const DEFAULT: u64 = 3600;

    fn get() -> u64 {
        match Self::get_raw() {
            Ok(value) => value.parse().unwrap_or(Self::DEFAULT),
            Err(_) => Self::DEFAULT,
        }
    }
}
