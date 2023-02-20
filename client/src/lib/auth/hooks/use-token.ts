import { AuthContextData } from "../provider/context";
import { useAuth } from "./use-auth";

type TokenState = [
  AuthContextData["token"],
  (token: AuthContextData["token"]) => void
];

export const useToken = (): TokenState => {
  const [auth, setAuth] = useAuth();

  const token = auth.token;
  const setToken = (token: string | null) => {
    setAuth((auth) => ({ ...auth, token }));
  };

  return [token, setToken];
};
