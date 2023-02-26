import { createContext } from "react";
import { type TokenProvider } from "../../common/token";
import { WhoAmIResult } from "../api/whoami";

/**
 * The context data for the auth context.
 */
export interface AuthContextData {
  /**
   * Token provider for the auth context.
   */
  tokenProvider: TokenProvider;

  /**
   * The current user, if available.
   */
  whoAmI: WhoAmIResult | null;
}

/**
 * The context parameters for the auth context.
 */
export type ContextParams = [
  AuthContextData,
  (f: (acd: AuthContextData) => AuthContextData) => void
];

/**
 * The auth context.
 */
export const context = createContext<ContextParams | null>(null);
