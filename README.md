## Prepare

```shell
node json-schema/run.js

cargo install sea-orm-cli
```

## Server Environment Variables

The server can be configured using environment variables.

| Name                            | Description                                                 | Default                |
|---------------------------------|-------------------------------------------------------------|------------------------|
| `AUTH_TOKEN_SIZE`               | The size of the authentication token, in bytes.             | `32`                   |
| `AUTH_TOKEN_TTL`                | The time to live of the authentication token, in seconds.   | `3600`                 |
| `DB_CONNECTION_STRING`          | The connection string to use to connect to the database.    | `sqlite://database.db` |
| `ENCRYPTION_KEY_PATH`           | The file-path to the encryption key.                        | `encryption.key`       |
| `REDIS_CACHE_CONNECTION_STRING` | The connection string to use to connect to the Redis cache. | `redis://cache`        |
| `REFRESH_TOKEN_SIZE`            | The size of the refresh token, in bytes.                    | `32`                   |
| `REFRESH_TOKEN_TTL`             | The time to live of the refresh token, in seconds.          | `604800`               |
