import { LoginApi, LoginApiOptions, LoginController } from "../api";
import { useAuth } from "./use-auth";

export const useLoginApi = (
  options?: LoginApiOptions
): LoginController["login"] => {
  const [auth] = useAuth();

  if (!auth) {
    throw new Error("useLoginApi must be used within a LoginProvider");
  }

  const api = new LoginApi(options);

  return api.login.bind(api);
};
