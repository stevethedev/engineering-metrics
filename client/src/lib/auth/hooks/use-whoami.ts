import { WhoAmIApi, WhoAmIApiOptions, WhoAmIResult } from "../api/whoami";
import { useToken } from "./use-token";
import { useEffect } from "react";
import { useAuth } from "./use-auth";
import { AuthContextData } from "../provider/context";

/**
 * Hook to use the whoami state.
 * @param options Options for the whoami API.
 */
export const useWhoAmI = (
  options: WhoAmIApiOptions = {}
): [WhoAmIResult | null, (whoAmI: WhoAmIResult | null) => void] => {
  const [token] = useToken(options.requesterOptions?.tokenProvider);
  const [auth, setAuth] = useAuth();
  const whoAmI = auth.whoAmI ?? null;

  const setWhoami = (whoAmI: AuthContextData["whoAmI"] | null) => {
    setAuth((auth) => ({
      ...auth,
      whoAmI,
    }));
  };

  useEffect(() => {
    const api = new WhoAmIApi(options);
    api
      .whoAmI(token)
      .then((result) => {
        setWhoami(result);
      })
      .catch(() => {
        setWhoami(null);
      });
  }, [token]);

  return [whoAmI, setWhoami];
};
