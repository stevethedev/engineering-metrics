import { FC, ReactNode, useState } from "react";
import { AuthContextData, context } from "./context";
import { DeepPartial } from "ts-essentials";

export interface AuthProviderProps {
  children: ReactNode;

  data?: DeepPartial<AuthContextData> | null;
}

const fillData = (
  data: DeepPartial<AuthContextData> | null
): AuthContextData => ({
  token: data?.token ?? null,
});

export const Provider: FC<AuthProviderProps> = ({ children, data = null }) => {
  const filledData = fillData(data);
  const [auth, setAuth] = useState<AuthContextData>(filledData);

  return (
    <context.Provider value={[auth, setAuth]}>{children}</context.Provider>
  );
};
