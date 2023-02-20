export const apiBaseUrl = process.env.API_BASE_URL || "/api";
export const apiAuthUrl = process.env.API_AUTH_URL || `${apiBaseUrl}/auth`;
export const apiAuthLoginUrl =
  process.env.API_AUTH_LOGIN_URL || `${apiAuthUrl}/login`;
