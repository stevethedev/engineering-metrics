import { LogoutButton} from "./logout-button";
import { FC, ReactNode } from "react";
import { useLogoutApi } from "../../lib/auth";

export interface LogoutControllerProps {
  children?: ReactNode;
}

export const LogoutController: FC<LogoutControllerProps> = ({ children }) => {
  const handleLogout = useLogoutApi();

  return <LogoutButton onLogout={handleLogout}>{children}</LogoutButton>
};
