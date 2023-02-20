import { useLoginApi } from "../../lib/auth/hooks/use-login-api";
import { LoginForm } from "./login-form";
import { useToken } from "../../lib/auth";

export const LoginController = () => {
  const login = useLoginApi();
  const [_, setToken] = useToken();

  const handleLogin = async (props: { username: string; password: string }) => {
    const result = await login(props);
    setToken(result?.token ?? null);
  }

  return (
    <LoginForm onSubmit={handleLogin} />
  )
}
