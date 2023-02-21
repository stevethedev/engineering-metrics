import { FC } from "react";
import { useWhoAmI } from "../../lib/auth/hooks/use-whoami";

export const WhoAmI: FC = () => {
  const [whoami] = useWhoAmI();

  if (whoami) {
    return <div>You are logged in as {whoami.username}</div>;
  }
  return null;
};
