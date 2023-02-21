import { LogoutApi, LogoutApiOptions, LogoutController } from "../api";
import { useToken } from "./use-token";

/**
 * Hook for using the login API.
 * @param options Options for the login API.
 */
export const useLogoutApi = (
  options?: LogoutApiOptions
): LogoutController["logout"] => {
  const [, setToken] = useToken();
  const api = new LogoutApi(options);

  return async (token) => {
    const promise = api.logout(token);
    setToken(null);
    await promise;
  };
};
