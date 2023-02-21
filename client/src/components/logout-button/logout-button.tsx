import { FC, ReactNode, useCallback } from "react";

export interface LogoutButtonProps {
  children?: ReactNode;
  onLogout: () => void;
}

export const LogoutButton: FC<LogoutButtonProps> = ({ children = "Logout", onLogout }) => {
  const handleLogout = useCallback(() => {
    onLogout();
  }, [onLogout]);

  return <button onClick={handleLogout}>{children}</button>;
}
