/**
 * Takes an environment variable and returns the value if it is defined and not
 * empty. Otherwise, returns the default value.
 * @param value
 * @param defaultValue
 */
const getEnv = (value: string | undefined | null, defaultValue: string) => {
  if (value === undefined || value === null || value === "") {
    return defaultValue;
  }
  return value;
};

/**
 * Base URL for sending requests to the API.
 */
export const apiBaseUrl = getEnv(process.env.API_BASE_URL, "/api");

/**
 * Base URL for sending requests to the authentication API.
 */
export const apiAuthUrl = getEnv(
  process.env.API_AUTH_URL,
  `${apiBaseUrl}/auth`
);

/**
 * URL for sending login requests to the authentication API.
 */
export const apiAuthLoginUrl = getEnv(
  process.env.API_AUTH_LOGIN_URL,
  `${apiAuthUrl}/login`
);

/**
 * URL for sending logout requests to the authentication API.
 */
export const apiAuthLogoutUrl = getEnv(
  process.env.API_AUTH_LOGOUT_URL,
  `${apiAuthUrl}/logout`
);

/**
 * URL for sending whoami requests to the authentication API.
 */

export const apiAuthWhoAmIUrl = getEnv(
  process.env.API_AUTH_WHOAMI_URL,
  `${apiAuthUrl}/whoami`
);

/**
 * Name of the local storage key for the API token.
 */
export const apiTokenKey = getEnv(process.env.API_TOKEN_KEY, "token");
