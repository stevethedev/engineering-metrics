import { createContext, Dispatch, SetStateAction } from "react";

export interface AuthContextData {
  readonly token: string | null;
}

type ContextParams = [
  AuthContextData | null,
  Dispatch<SetStateAction<AuthContextData | null>>
];

export const context = createContext<ContextParams>(null);
