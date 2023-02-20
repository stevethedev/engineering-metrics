import { useContext } from "react";
import { context } from "../provider/context";

export const useAuth = () => {
  const auth = useContext(context);

  if (!auth) {
    throw new Error("useAuth must be used within an AuthProvider");
  }

  return auth;
};
