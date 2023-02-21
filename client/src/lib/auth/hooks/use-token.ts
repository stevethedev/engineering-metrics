import { useAuth } from "./use-auth";
import { TokenProvider } from "../../common/token";

type TokenState = [string | null, (token: string | null) => void];

/**
 * Hook to use the token state.
 * @param tokenProvider Token provider to use. If not provided, the token
 * provider from the auth context will be used.
 */
export const useToken = (
  tokenProvider: TokenProvider | null = null
): TokenState => {
  const [auth, setAuth] = useAuth();
  const manager = tokenProvider ?? auth.tokenProvider;
  const setToken = (token: string | null) => {
    manager.setToken(token);
    setAuth((auth) => ({ ...auth, tokenProvider: manager }));
  };
  return [manager.token, setToken];
};
