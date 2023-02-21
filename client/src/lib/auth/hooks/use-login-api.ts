import { LoginApi, LoginApiOptions, LoginController } from "../api";

/**
 * Hook for using the login API.
 * @param options Options for the login API.
 */
export const useLoginApi = (
  options?: LoginApiOptions
): LoginController["login"] => {
  const api = new LoginApi(options);
  return api.login.bind(api);
};
