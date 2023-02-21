import { FC } from "react";
import { LoginController } from "../../components/login-form";
import { useWhoAmI } from "../../lib/auth/hooks/use-whoami";
import { WhoAmI } from "../../components/whoami";

/**
 * The login page.
 */
export const LoginPage: FC = () => {
  const [whoami] = useWhoAmI();

  if (whoami) {
    return <WhoAmI />;
  }

  return <LoginController />;
};
