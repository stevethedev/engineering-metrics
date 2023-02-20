import { FC } from "react";
import { LoginController } from "../../components/login-form";

export interface LoginPage {}

export const LoginPage: FC<LoginPage> = () => {
  return <LoginController />;
};
