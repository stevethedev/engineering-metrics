import { useContext } from "react";
import { context, ContextParams } from "../provider/context";

/**
 * Hook to use the auth context
 *
 * @returns The auth context.
 */
export const useAuth = (): ContextParams => {
  const auth = useContext(context);

  if (!auth) {
    throw new Error("useAuth must be used within an AuthProvider");
  }

  return auth;
};
