import { FC } from "react";
import { LoginController } from "../../components/containers/login-form";
import { useWhoAmI } from "../../lib/auth";
import { WhoAmI } from "../../components/containers/whoami";
import { LogoutController } from "../../components/containers/logout-button";

/**
 * The login page.
 */
export const LoginPage: FC = () => {
  const [whoami] = useWhoAmI();

  if (whoami) {
    return (
      <div>
        <WhoAmI />
        <LogoutController />
      </div>
    );
  }

  return <LoginController />;
};
