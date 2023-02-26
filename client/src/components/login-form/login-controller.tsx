import { useLoginApi, useToken } from "../../lib/auth";
import { LoginForm } from "./login-form";

export const LoginController = () => {
  const login = useLoginApi();
  const [, setToken] = useToken();

  const handleLogin = (props: { username: string; password: string }) => {
    login(props).then(
      (result) => {
        setToken(result?.token ?? null, result?.tokenExpires ?? 0);
      },
      () => {
        setToken(null, 0);
      }
    );
  };

  return <LoginForm onSubmit={handleLogin} />;
};
