import { useLoginApi } from "../../lib/auth/hooks/use-login-api";
import { LoginForm } from "./login-form";
import { useToken } from "../../lib/auth";

export const LoginController = () => {
  const login = useLoginApi();
  const [, setToken] = useToken();

  const handleLogin = (props: { username: string; password: string }) => {
    login(props).then(
      (result) => {
        setToken(result?.token ?? null);
      },
      () => {
        setToken(null);
      }
    );
  };

  return <LoginForm onSubmit={handleLogin} />;
};
