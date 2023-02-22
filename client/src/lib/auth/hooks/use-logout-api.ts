import { LogoutApi, LogoutApiOptions, LogoutController } from "../api";
import { useToken } from "./use-token";
import { useCallback, useMemo } from "react";

/**
 * Hook for using the login API.
 * @param options Options for the login API.
 */
export const useLogoutApi = (
  options?: LogoutApiOptions
): LogoutController["logout"] => {
  const [, setToken] = useToken();
  const api = useMemo(() => new LogoutApi(options), [JSON.stringify(options)]);

  return useCallback(async (token) => {
    const promise = api.logout(token);
    setToken(null);
    await promise;
  }, []);
};
