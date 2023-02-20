# Client

## Build Environment Variables

| Variable             | Maps to           | Description                                | Default               |
|----------------------|-------------------|--------------------------------------------|-----------------------|
| `API_BASE_URL`       | `apiBaseUrl`      | The base URL of the API server.            | `/api`                |
| `API_AUTH_URL`       | `apiAuthUrl`      | The base URL of the authentication server. | `${apiBaseUrl}/auth`  |
| `API_AUTH_LOGIN_URL` | `apiAuthLoginUrl` | The URL of the login endpoint.             | `${apiAuthUrl}/login` |
