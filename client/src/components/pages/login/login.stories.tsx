import { Login, LoginProps } from "./login";

export default {
  title: "Pages/Login",
  component: Login,
};

export const Default = (args: LoginProps) => <Login {...args} />;
Default.storyName = "Login";
