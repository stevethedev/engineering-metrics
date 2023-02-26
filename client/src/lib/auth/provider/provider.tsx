import { FC, ReactNode, useState } from "react";
import { AuthContextData, context } from "./context";
import { TokenManager } from "../../common/token";

export interface AuthProviderProps {
  /**
   * The children to render.
   */
  children: ReactNode;

  /**
   * The initial data to use.
   */
  data?: Partial<AuthContextData>;
}

/**
 * Fills in the data with default values.
 * @param data The data to fill.
 */
const fillData = (
  data: Partial<AuthContextData> | null = null
): AuthContextData => {
  return {
    whoAmI: data?.whoAmI ?? null,
    tokenProvider: data?.tokenProvider ?? new TokenManager(),
  };
};

/**
 * The provider for the auth context.
 */
export const Provider: FC<AuthProviderProps> = ({ children, data = null }) => {
  const filledData = fillData(data);
  const [auth, setAuth] = useState<AuthContextData>(filledData);

  return (
    <context.Provider value={[auth, setAuth]}>{children}</context.Provider>
  );
};
