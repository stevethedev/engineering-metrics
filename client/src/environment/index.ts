/**
 * Base URL for sending requests to the API.
 */
export const apiBaseUrl = process.env.API_BASE_URL ?? "/api";

/**
 * Base URL for sending requests to the authentication API.
 */
export const apiAuthUrl = process.env.API_AUTH_URL ?? `${apiBaseUrl}/auth`;

/**
 * URL for sending login requests to the authentication API.
 */
export const apiAuthLoginUrl =
  process.env.API_AUTH_LOGIN_URL ?? `${apiAuthUrl}/login`;

/**
 * URL for sending whoami requests to the authentication API.
 */

export const apiAuthWhoAmIUrl =
  process.env.API_AUTH_WHOAMI_URL ?? `${apiAuthUrl}/whoami`;

/**
 * Name of the local storage key for the API token.
 */
export const apiTokenKey = process.env.API_TOKEN_KEY ?? "token";
